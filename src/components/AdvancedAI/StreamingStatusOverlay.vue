<template>
  <div v-if="isVisible" class="streaming-overlay">
    <div class="streaming-content">
      <!-- Header -->
      <div class="streaming-header">
        <div class="status-info">
          <div class="status-indicator">
            <i :class="statusIcon" :style="{ color: statusColor }"></i>
            <span class="status-text">{{ statusText }}</span>
          </div>
          <div class="progress-info">
            <span class="progress-text">{{ progressText }}</span>
            <button 
              v-if="canCancel"
              @click="handleCancel"
              class="cancel-button"
              :disabled="isCancelling"
            >
              <i :class="isCancelling ? 'fas fa-spinner fa-spin' : 'fas fa-times'"></i>
              {{ isCancelling ? 'Cancelling...' : 'Cancel' }}
            </button>
          </div>
        </div>
      </div>

      <!-- Progress Bar -->
      <div class="progress-container">
        <div class="progress-bar">
          <div 
            class="progress-fill"
            :style="{ width: `${progress}%`, backgroundColor: statusColor }"
          ></div>
        </div>
        <div class="progress-percentage">{{ Math.round(progress) }}%</div>
      </div>

      <!-- Streaming Content -->
      <div v-if="streamingContent" class="streaming-text-container">
        <div class="streaming-text-header">
          <h4>Generated Content</h4>
          <div class="text-stats">
            <span class="word-count">{{ wordCount }} words</span>
            <span class="char-count">{{ charCount }} characters</span>
          </div>
        </div>
        
        <div class="streaming-text" ref="streamingTextRef">
          <div class="generated-text">{{ streamingContent }}</div>
          <div v-if="isGenerating" class="typing-cursor">|</div>
        </div>
        
        <!-- Text Actions -->
        <div class="text-actions">
          <button 
            @click="copyToClipboard"
            class="action-button"
            :disabled="!streamingContent"
          >
            <i class="fas fa-copy"></i>
            Copy
          </button>
          
          <button 
            @click="insertIntoDocument"
            class="action-button"
            :disabled="!streamingContent"
          >
            <i class="fas fa-plus"></i>
            Insert
          </button>
          
          <button 
            @click="saveAsSnippet"
            class="action-button"
            :disabled="!streamingContent"
          >
            <i class="fas fa-bookmark"></i>
            Save
          </button>
        </div>
      </div>

      <!-- Error Display -->
      <div v-if="error" class="error-container">
        <div class="error-header">
          <i class="fas fa-exclamation-triangle"></i>
          <h4>Generation Error</h4>
        </div>
        <div class="error-message">{{ error }}</div>
        <div class="error-actions">
          <button @click="handleRetry" class="retry-button">
            <i class="fas fa-redo"></i>
            Retry
          </button>
          <button @click="handleClose" class="close-error-button">
            <i class="fas fa-times"></i>
            Close
          </button>
        </div>
      </div>

      <!-- Generation Stats -->
      <div v-if="stats" class="stats-container">
        <div class="stats-grid">
          <div class="stat-item">
            <span class="stat-label">Duration</span>
            <span class="stat-value">{{ formatDuration(stats.duration) }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">Tokens/sec</span>
            <span class="stat-value">{{ stats.tokensPerSecond?.toFixed(1) || 'N/A' }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">Credits Used</span>
            <span class="stat-value">{{ stats.creditsUsed || 'N/A' }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">Model</span>
            <span class="stat-value">{{ stats.model || 'N/A' }}</span>
          </div>
        </div>
      </div>

      <!-- Close Button -->
      <div v-if="isComplete" class="close-container">
        <button @click="handleClose" class="close-button">
          <i class="fas fa-check"></i>
          Done
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, onMounted, onUnmounted } from 'vue';
import { useAdvancedAIStore } from '../../stores/advancedAIStore';
import { useProjectStore } from '../../stores/projectStore';
import type { StreamingStatus } from '../../types/advancedAI';

// Props
interface Props {
  visible?: boolean;
  streamingId?: string;
}

const props = withDefaults(defineProps<Props>(), {
  visible: false,
  streamingId: ''
});

// Emits
interface Emits {
  close: [];
  cancel: [];
  retry: [];
  contentReady: [content: string];
}

const emit = defineEmits<Emits>();

// Stores
const advancedAIStore = useAdvancedAIStore();
const projectStore = useProjectStore();

// Reactive state
const streamingTextRef = ref<HTMLElement>();
const isCancelling = ref(false);
const startTime = ref<number>(0);
const updateInterval = ref<number>();

// Computed properties
const isVisible = computed(() => {
  return props.visible && (isGenerating.value || isComplete.value || !!error.value);
});

const streamingStatus = computed(() => {
  return advancedAIStore.streamingStatus;
});

const isGenerating = computed(() => {
  return streamingStatus.value?.status === 'generating';
});

const isComplete = computed(() => {
  return streamingStatus.value?.status === 'completed';
});

const error = computed(() => {
  return streamingStatus.value?.error;
});

const streamingContent = computed(() => {
  return streamingStatus.value?.content || '';
});

const progress = computed(() => {
  return streamingStatus.value?.progress || 0;
});

const statusIcon = computed(() => {
  switch (streamingStatus.value?.status) {
    case 'initializing':
      return 'fas fa-hourglass-start';
    case 'generating':
      return 'fas fa-spinner fa-spin';
    case 'completed':
      return 'fas fa-check-circle';
    case 'cancelled':
      return 'fas fa-times-circle';
    case 'error':
      return 'fas fa-exclamation-triangle';
    default:
      return 'fas fa-circle';
  }
});

const statusColor = computed(() => {
  switch (streamingStatus.value?.status) {
    case 'initializing':
      return '#fbbf24'; // yellow
    case 'generating':
      return '#3b82f6'; // blue
    case 'completed':
      return '#10b981'; // green
    case 'cancelled':
      return '#6b7280'; // gray
    case 'error':
      return '#ef4444'; // red
    default:
      return '#6b7280';
  }
});

const statusText = computed(() => {
  switch (streamingStatus.value?.status) {
    case 'initializing':
      return 'Initializing generation...';
    case 'generating':
      return 'Generating content...';
    case 'completed':
      return 'Generation completed';
    case 'cancelled':
      return 'Generation cancelled';
    case 'error':
      return 'Generation failed';
    default:
      return 'Unknown status';
  }
});

const progressText = computed(() => {
  const status = streamingStatus.value;
  if (!status) return '';
  
  if (status.estimatedTokens && status.currentTokens) {
    return `${status.currentTokens} / ${status.estimatedTokens} tokens`;
  }
  
  if (status.currentTokens) {
    return `${status.currentTokens} tokens generated`;
  }
  
  return '';
});

const canCancel = computed(() => {
  return isGenerating.value && !isCancelling.value;
});

const wordCount = computed(() => {
  if (!streamingContent.value) return 0;
  return streamingContent.value.trim().split(/\s+/).filter(word => word.length > 0).length;
});

const charCount = computed(() => {
  return streamingContent.value.length;
});

const stats = computed(() => {
  const status = streamingStatus.value;
  if (!status || !isComplete.value) return null;
  
  const duration = status.endTime && status.startTime 
    ? status.endTime - status.startTime 
    : Date.now() - startTime.value;
  
  const tokensPerSecond = status.currentTokens && duration > 0
    ? (status.currentTokens / (duration / 1000))
    : undefined;
  
  return {
    duration,
    tokensPerSecond,
    creditsUsed: status.creditsUsed,
    model: status.model
  };
});

// Methods
const handleCancel = async () => {
  if (!canCancel.value) return;
  
  isCancelling.value = true;
  
  try {
    await advancedAIStore.cancelStreaming(props.streamingId);
    emit('cancel');
  } catch (error) {
    console.error('Failed to cancel streaming:', error);
  } finally {
    isCancelling.value = false;
  }
};

const handleRetry = () => {
  emit('retry');
};

const handleClose = () => {
  emit('close');
};

const copyToClipboard = async () => {
  if (!streamingContent.value) return;
  
  try {
    await navigator.clipboard.writeText(streamingContent.value);
    // Could show a toast notification here
  } catch (error) {
    console.error('Failed to copy to clipboard:', error);
  }
};

const insertIntoDocument = () => {
  if (!streamingContent.value) return;
  
  // Insert content into current document
  const currentDoc = projectStore.currentDocument;
  if (currentDoc) {
    // This would typically trigger a document update
    emit('contentReady', streamingContent.value);
  }
};

const saveAsSnippet = async () => {
  if (!streamingContent.value) return;
  
  try {
    // Save as snippet (would need to implement snippet functionality)
    // For now, just copy to clipboard
    await copyToClipboard();
  } catch (error) {
    console.error('Failed to save snippet:', error);
  }
};

const formatDuration = (ms: number): string => {
  const seconds = Math.floor(ms / 1000);
  const minutes = Math.floor(seconds / 60);
  
  if (minutes > 0) {
    return `${minutes}m ${seconds % 60}s`;
  }
  
  return `${seconds}s`;
};

const scrollToBottom = () => {
  if (streamingTextRef.value) {
    streamingTextRef.value.scrollTop = streamingTextRef.value.scrollHeight;
  }
};

// Watchers
watch(streamingContent, () => {
  nextTick(() => {
    scrollToBottom();
  });
});

watch(() => props.visible, (visible) => {
  if (visible) {
    startTime.value = Date.now();
  }
});

watch(isGenerating, (generating) => {
  if (generating) {
    // Start update interval for real-time stats
    updateInterval.value = window.setInterval(() => {
      // Force reactivity update for duration calculation
      startTime.value = startTime.value;
    }, 1000);
  } else {
    // Clear update interval
    if (updateInterval.value) {
      clearInterval(updateInterval.value);
      updateInterval.value = undefined;
    }
  }
});

// Lifecycle
onMounted(() => {
  startTime.value = Date.now();
});

onUnmounted(() => {
  if (updateInterval.value) {
    clearInterval(updateInterval.value);
  }
});
</script>

<style scoped>
.streaming-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.8);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
  padding: 1rem;
}

.streaming-content {
  background: var(--bg-primary);
  border-radius: 8px;
  width: 100%;
  max-width: 800px;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
  overflow: hidden;
}

.streaming-header {
  padding: 1.5rem;
  border-bottom: 1px solid var(--border-color);
  background: var(--bg-secondary);
}

.status-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1rem;
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.status-indicator i {
  font-size: 1.25rem;
}

.status-text {
  font-weight: 500;
  color: var(--text-primary);
}

.progress-info {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.progress-text {
  font-size: 0.875rem;
  color: var(--text-secondary);
}

.cancel-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: transparent;
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 0.875rem;
}

.cancel-button:hover:not(:disabled) {
  background: var(--bg-hover);
  border-color: #ef4444;
  color: #ef4444;
}

.cancel-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.progress-container {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem 1.5rem;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
}

.progress-bar {
  flex: 1;
  height: 8px;
  background: var(--bg-tertiary);
  border-radius: 4px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  transition: width 0.3s ease;
  border-radius: 4px;
}

.progress-percentage {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--text-primary);
  min-width: 40px;
  text-align: right;
}

.streaming-text-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.streaming-text-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.5rem 0.5rem;
}

.streaming-text-header h4 {
  margin: 0;
  color: var(--text-primary);
}

.text-stats {
  display: flex;
  gap: 1rem;
  font-size: 0.875rem;
  color: var(--text-secondary);
}

.streaming-text {
  flex: 1;
  padding: 0.5rem 1.5rem;
  overflow-y: auto;
  position: relative;
}

.generated-text {
  font-family: var(--font-mono);
  line-height: 1.6;
  color: var(--text-primary);
  white-space: pre-wrap;
  word-wrap: break-word;
}

.typing-cursor {
  display: inline;
  animation: blink 1s infinite;
  color: var(--accent-color);
  font-weight: bold;
}

@keyframes blink {
  0%, 50% { opacity: 1; }
  51%, 100% { opacity: 0; }
}

.text-actions {
  display: flex;
  gap: 0.5rem;
  padding: 1rem 1.5rem;
  border-top: 1px solid var(--border-color);
  background: var(--bg-secondary);
}

.action-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: transparent;
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 0.875rem;
}

.action-button:hover:not(:disabled) {
  background: var(--bg-hover);
  border-color: var(--accent-color);
}

.action-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.error-container {
  padding: 1.5rem;
  background: #fef2f2;
  border-top: 1px solid #fecaca;
}

.error-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
}

.error-header i {
  color: #ef4444;
}

.error-header h4 {
  margin: 0;
  color: #dc2626;
}

.error-message {
  color: #991b1b;
  margin-bottom: 1rem;
  line-height: 1.5;
}

.error-actions {
  display: flex;
  gap: 0.5rem;
}

.retry-button,
.close-error-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  border: 1px solid #dc2626;
  border-radius: 4px;
  background: transparent;
  color: #dc2626;
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 0.875rem;
}

.retry-button:hover {
  background: #dc2626;
  color: white;
}

.close-error-button:hover {
  background: #f3f4f6;
}

.stats-container {
  padding: 1rem 1.5rem;
  background: var(--bg-secondary);
  border-top: 1px solid var(--border-color);
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 1rem;
}

.stat-item {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.stat-label {
  font-size: 0.75rem;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.stat-value {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--text-primary);
}

.close-container {
  padding: 1.5rem;
  text-align: center;
  background: var(--bg-secondary);
  border-top: 1px solid var(--border-color);
}

.close-button {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 2rem;
  border: none;
  border-radius: 4px;
  background: var(--accent-color);
  color: white;
  cursor: pointer;
  transition: all 0.2s ease;
  font-weight: 500;
}

.close-button:hover {
  background: var(--accent-color-hover);
}

/* Responsive design */
@media (max-width: 768px) {
  .streaming-overlay {
    padding: 0.5rem;
  }
  
  .streaming-content {
    max-height: 95vh;
  }
  
  .streaming-header,
  .progress-container,
  .streaming-text-header,
  .text-actions,
  .stats-container,
  .close-container {
    padding-left: 1rem;
    padding-right: 1rem;
  }
  
  .status-info {
    flex-direction: column;
    align-items: stretch;
    gap: 0.5rem;
  }
  
  .progress-info {
    justify-content: space-between;
  }
  
  .text-actions {
    flex-wrap: wrap;
  }
  
  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}
</style>