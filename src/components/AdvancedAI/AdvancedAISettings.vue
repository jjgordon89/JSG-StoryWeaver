<template>
  <div class="advanced-ai-settings">
    <!-- Header -->
    <div class="settings-header">
      <h3>Advanced AI Settings</h3>
      <div class="header-actions">
        <button @click="resetToDefaults" class="reset-button">
          <i class="fas fa-undo"></i>
          Reset to Defaults
        </button>
        <button @click="exportSettings" class="export-button">
          <i class="fas fa-download"></i>
          Export
        </button>
        <button @click="importSettings" class="import-button">
          <i class="fas fa-upload"></i>
          Import
        </button>
      </div>
    </div>

    <!-- Settings Content -->
    <div class="settings-content">
      <!-- General Settings -->
      <div class="settings-section">
        <h4>General Settings</h4>
        
        <div class="setting-group">
          <label class="setting-label">
            Default Prose Mode
            <select v-model="settings.general.defaultProseMode" class="setting-select">
              <option value="">Select default mode</option>
              <option 
                v-for="mode in proseModes" 
                :key="mode.id"
                :value="mode.id"
              >
                {{ mode.name }}
              </option>
            </select>
          </label>
          <div class="setting-hint">
            The prose mode that will be selected by default for new generations.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Auto-save Generated Content
            <input 
              type="checkbox" 
              v-model="settings.general.autoSave"
              class="setting-checkbox"
            >
          </label>
          <div class="setting-hint">
            Automatically save generated text and images to your project.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Show Advanced Options by Default
            <input 
              type="checkbox" 
              v-model="settings.general.showAdvancedOptions"
              class="setting-checkbox"
            >
          </label>
          <div class="setting-hint">
            Display advanced generation options without needing to expand them.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Enable Streaming Generation
            <input 
              type="checkbox" 
              v-model="settings.general.enableStreaming"
              class="setting-checkbox"
            >
          </label>
          <div class="setting-hint">
            Show text as it's being generated in real-time (uses more resources).
          </div>
        </div>
      </div>

      <!-- Generation Settings -->
      <div class="settings-section">
        <h4>Generation Settings</h4>
        
        <div class="setting-group">
          <label class="setting-label">
            Default Context Length
            <div class="range-input-container">
              <input 
                type="range" 
                v-model="settings.generation.defaultContextLength"
                min="500"
                max="8000"
                step="100"
                class="setting-range"
              >
              <span class="range-value">{{ settings.generation.defaultContextLength }} words</span>
            </div>
          </label>
          <div class="setting-hint">
            How much context from your story to include in generations.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Default Output Length
            <div class="range-input-container">
              <input 
                type="range" 
                v-model="settings.generation.defaultOutputLength"
                min="50"
                max="1000"
                step="25"
                class="setting-range"
              >
              <span class="range-value">{{ settings.generation.defaultOutputLength }} words</span>
            </div>
          </label>
          <div class="setting-hint">
            Target length for generated text.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Default Creativity Level
            <div class="range-input-container">
              <input 
                type="range" 
                v-model="settings.generation.defaultCreativity"
                min="0.1"
                max="1.0"
                step="0.1"
                class="setting-range"
              >
              <span class="range-value">{{ (settings.generation.defaultCreativity * 100).toFixed(0) }}%</span>
            </div>
          </label>
          <div class="setting-hint">
            Higher values produce more creative but less predictable results.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Enable Cliché Detection
            <input 
              type="checkbox" 
              v-model="settings.generation.enableClicheDetection"
              class="setting-checkbox"
            >
          </label>
          <div class="setting-hint">
            Automatically detect and highlight potential clichés in generated content.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Ultra-Creative Mode Threshold
            <div class="range-input-container">
              <input 
                type="range" 
                v-model="settings.generation.ultraCreativeThreshold"
                min="0.7"
                max="1.0"
                step="0.05"
                class="setting-range"
              >
              <span class="range-value">{{ (settings.generation.ultraCreativeThreshold * 100).toFixed(0) }}%</span>
            </div>
          </label>
          <div class="setting-hint">
            Creativity level at which ultra-creative mode is automatically enabled.
          </div>
        </div>
      </div>

      <!-- Saliency Engine Settings -->
      <div class="settings-section">
        <h4>Saliency Engine Settings</h4>
        
        <div class="setting-group">
          <label class="setting-label">
            Enable Saliency Engine
            <input 
              type="checkbox" 
              v-model="settings.saliency.enabled"
              class="setting-checkbox"
            >
          </label>
          <div class="setting-hint">
            Use AI to identify and prioritize the most important story elements.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Auto-build Context
            <input 
              type="checkbox" 
              v-model="settings.saliency.autoBuildContext"
              class="setting-checkbox"
            >
          </label>
          <div class="setting-hint">
            Automatically build saliency context when generating content.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Context Refresh Interval
            <select v-model="settings.saliency.refreshInterval" class="setting-select">
              <option value="never">Never</option>
              <option value="daily">Daily</option>
              <option value="weekly">Weekly</option>
              <option value="monthly">Monthly</option>
              <option value="manual">Manual Only</option>
            </select>
          </label>
          <div class="setting-hint">
            How often to automatically refresh the saliency context.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Include Character Relationships
            <input 
              type="checkbox" 
              v-model="settings.saliency.includeRelationships"
              class="setting-checkbox"
            >
          </label>
          <div class="setting-hint">
            Include character relationships in saliency analysis.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Include Plot Threads
            <input 
              type="checkbox" 
              v-model="settings.saliency.includePlotThreads"
              class="setting-checkbox"
            >
          </label>
          <div class="setting-hint">
            Include active plot threads in saliency analysis.
          </div>
        </div>
      </div>

      <!-- Image Generation Settings -->
      <div class="settings-section">
        <h4>Image Generation Settings</h4>
        
        <div class="setting-group">
          <label class="setting-label">
            Default Art Style
            <select v-model="settings.images.defaultStyle" class="setting-select">
              <option value="realistic">Realistic</option>
              <option value="artistic">Artistic</option>
              <option value="fantasy">Fantasy</option>
              <option value="sci-fi">Sci-Fi</option>
              <option value="historical">Historical</option>
              <option value="minimalist">Minimalist</option>
              <option value="abstract">Abstract</option>
            </select>
          </label>
          <div class="setting-hint">
            Default art style for image generation.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Default Resolution
            <select v-model="settings.images.defaultResolution" class="setting-select">
              <option value="512x512">512×512 (Square)</option>
              <option value="768x512">768×512 (Landscape)</option>
              <option value="512x768">512×768 (Portrait)</option>
              <option value="1024x1024">1024×1024 (High Quality Square)</option>
            </select>
          </label>
          <div class="setting-hint">
            Default image resolution and aspect ratio.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Default Quality
            <select v-model="settings.images.defaultQuality" class="setting-select">
              <option value="draft">Draft (Fast, Lower Cost)</option>
              <option value="standard">Standard (Balanced)</option>
              <option value="high">High (Slower, Higher Cost)</option>
            </select>
          </label>
          <div class="setting-hint">
            Default quality level for image generation.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Auto-enhance Prompts
            <input 
              type="checkbox" 
              v-model="settings.images.autoEnhancePrompts"
              class="setting-checkbox"
            >
          </label>
          <div class="setting-hint">
            Automatically enhance image prompts with additional details.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Use Story Context
            <input 
              type="checkbox" 
              v-model="settings.images.useStoryContext"
              class="setting-checkbox"
            >
          </label>
          <div class="setting-hint">
            Include story context in image generation for consistency.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Auto-save Images
            <input 
              type="checkbox" 
              v-model="settings.images.autoSave"
              class="setting-checkbox"
            >
          </label>
          <div class="setting-hint">
            Automatically save generated images to your project gallery.
          </div>
        </div>
      </div>

      <!-- Brainstorming Settings -->
      <div class="settings-section">
        <h4>Brainstorming Settings</h4>
        
        <div class="setting-group">
          <label class="setting-label">
            Default Session Duration
            <div class="range-input-container">
              <input 
                type="range" 
                v-model="settings.brainstorming.defaultDuration"
                min="5"
                max="60"
                step="5"
                class="setting-range"
              >
              <span class="range-value">{{ settings.brainstorming.defaultDuration }} minutes</span>
            </div>
          </label>
          <div class="setting-hint">
            Default duration for brainstorming sessions.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Ideas per Generation
            <div class="range-input-container">
              <input 
                type="range" 
                v-model="settings.brainstorming.ideasPerGeneration"
                min="3"
                max="15"
                step="1"
                class="setting-range"
              >
              <span class="range-value">{{ settings.brainstorming.ideasPerGeneration }} ideas</span>
            </div>
          </label>
          <div class="setting-hint">
            Number of ideas to generate at once.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Auto-rate Ideas
            <input 
              type="checkbox" 
              v-model="settings.brainstorming.autoRate"
              class="setting-checkbox"
            >
          </label>
          <div class="setting-hint">
            Automatically provide initial ratings for generated ideas.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Include Story Context
            <input 
              type="checkbox" 
              v-model="settings.brainstorming.useStoryContext"
              class="setting-checkbox"
            >
          </label>
          <div class="setting-hint">
            Use current story context to generate relevant ideas.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Auto-export Keepers
            <input 
              type="checkbox" 
              v-model="settings.brainstorming.autoExportKeepers"
              class="setting-checkbox"
            >
          </label>
          <div class="setting-hint">
            Automatically export keeper ideas to the Story Bible.
          </div>
        </div>
      </div>

      <!-- Performance Settings -->
      <div class="settings-section">
        <h4>Performance Settings</h4>
        
        <div class="setting-group">
          <label class="setting-label">
            Cache Generated Content
            <input 
              type="checkbox" 
              v-model="settings.performance.cacheContent"
              class="setting-checkbox"
            >
          </label>
          <div class="setting-hint">
            Cache generated content to improve performance and reduce costs.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Preload Models
            <input 
              type="checkbox" 
              v-model="settings.performance.preloadModels"
              class="setting-checkbox"
            >
          </label>
          <div class="setting-hint">
            Preload AI models for faster generation (uses more memory).
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Background Processing
            <input 
              type="checkbox" 
              v-model="settings.performance.backgroundProcessing"
              class="setting-checkbox"
            >
          </label>
          <div class="setting-hint">
            Process non-critical tasks in the background.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Max Concurrent Generations
            <div class="range-input-container">
              <input 
                type="range" 
                v-model="settings.performance.maxConcurrentGenerations"
                min="1"
                max="5"
                step="1"
                class="setting-range"
              >
              <span class="range-value">{{ settings.performance.maxConcurrentGenerations }}</span>
            </div>
          </label>
          <div class="setting-hint">
            Maximum number of simultaneous AI generations.
          </div>
        </div>
      </div>

      <!-- Privacy & Data Settings -->
      <div class="settings-section">
        <h4>Privacy & Data Settings</h4>
        
        <div class="setting-group">
          <label class="setting-label">
            Store Generation History
            <input 
              type="checkbox" 
              v-model="settings.privacy.storeHistory"
              class="setting-checkbox"
            >
          </label>
          <div class="setting-hint">
            Keep a history of your AI generations for reference.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Share Anonymous Usage Data
            <input 
              type="checkbox" 
              v-model="settings.privacy.shareUsageData"
              class="setting-checkbox"
            >
          </label>
          <div class="setting-hint">
            Help improve StoryWeaver by sharing anonymous usage statistics.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Data Retention Period
            <select v-model="settings.privacy.dataRetention" class="setting-select">
              <option value="1month">1 Month</option>
              <option value="3months">3 Months</option>
              <option value="6months">6 Months</option>
              <option value="1year">1 Year</option>
              <option value="forever">Forever</option>
            </select>
          </label>
          <div class="setting-hint">
            How long to keep your AI generation data.
          </div>
        </div>
        
        <div class="setting-group">
          <label class="setting-label">
            Export Data Format
            <select v-model="settings.privacy.exportFormat" class="setting-select">
              <option value="json">JSON</option>
              <option value="csv">CSV</option>
              <option value="markdown">Markdown</option>
              <option value="pdf">PDF</option>
            </select>
          </label>
          <div class="setting-hint">
            Preferred format for data exports.
          </div>
        </div>
      </div>
    </div>

    <!-- Footer Actions -->
    <div class="settings-footer">
      <div class="footer-info">
        <span v-if="hasUnsavedChanges" class="unsaved-indicator">
          <i class="fas fa-circle"></i>
          Unsaved changes
        </span>
        <span class="last-saved">
          Last saved: {{ lastSavedTime }}
        </span>
      </div>
      
      <div class="footer-actions">
        <button @click="revertChanges" :disabled="!hasUnsavedChanges" class="revert-button">
          <i class="fas fa-undo"></i>
          Revert
        </button>
        
        <button @click="saveSettings" :disabled="!hasUnsavedChanges" class="save-button">
          <i class="fas fa-save"></i>
          Save Settings
        </button>
      </div>
    </div>

    <!-- Import Modal -->
    <div v-if="showImportModal" class="modal-overlay" @click="showImportModal = false">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h3>Import Settings</h3>
          <button @click="showImportModal = false" class="close-button">
            <i class="fas fa-times"></i>
          </button>
        </div>
        
        <div class="modal-body">
          <div class="import-options">
            <label class="import-option">
              <input type="radio" v-model="importMethod" value="file">
              Import from file
            </label>
            <label class="import-option">
              <input type="radio" v-model="importMethod" value="text">
              Paste settings JSON
            </label>
          </div>
          
          <div v-if="importMethod === 'file'" class="file-import">
            <input 
              type="file" 
              @change="handleFileImport"
              accept=".json"
              class="file-input"
            >
          </div>
          
          <div v-if="importMethod === 'text'" class="text-import">
            <textarea 
              v-model="importText"
              placeholder="Paste your settings JSON here..."
              class="import-textarea"
              rows="10"
            ></textarea>
          </div>
        </div>
        
        <div class="modal-footer">
          <button @click="showImportModal = false" class="cancel-button">
            Cancel
          </button>
          <button @click="processImport" :disabled="!canImport" class="import-confirm-button">
            Import Settings
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue';
import { useAdvancedAIStore } from '../../stores/advancedAIStore';

// Store
const advancedAIStore = useAdvancedAIStore();

// Reactive state
const settings = ref({
  general: {
    defaultProseMode: '',
    autoSave: true,
    showAdvancedOptions: false,
    enableStreaming: true
  },
  generation: {
    defaultContextLength: 2000,
    defaultOutputLength: 200,
    defaultCreativity: 0.7,
    enableClicheDetection: true,
    ultraCreativeThreshold: 0.85
  },
  saliency: {
    enabled: true,
    autoBuildContext: true,
    refreshInterval: 'weekly',
    includeRelationships: true,
    includePlotThreads: true
  },
  images: {
    defaultStyle: 'realistic',
    defaultResolution: '768x512',
    defaultQuality: 'standard',
    autoEnhancePrompts: true,
    useStoryContext: true,
    autoSave: true
  },
  brainstorming: {
    defaultDuration: 15,
    ideasPerGeneration: 5,
    autoRate: false,
    useStoryContext: true,
    autoExportKeepers: false
  },
  performance: {
    cacheContent: true,
    preloadModels: false,
    backgroundProcessing: true,
    maxConcurrentGenerations: 2
  },
  privacy: {
    storeHistory: true,
    shareUsageData: false,
    dataRetention: '6months',
    exportFormat: 'json'
  }
});

const originalSettings = ref<string>('');
const lastSavedTime = ref('Never');
const showImportModal = ref(false);
const importMethod = ref('file');
const importText = ref('');

// Computed properties
const proseModes = computed(() => advancedAIStore.proseModes);

const hasUnsavedChanges = computed(() => {
  return JSON.stringify(settings.value) !== originalSettings.value;
});

const canImport = computed(() => {
  if (importMethod.value === 'file') {
    return true; // File validation happens in handleFileImport
  }
  if (importMethod.value === 'text') {
    try {
      JSON.parse(importText.value);
      return true;
    } catch {
      return false;
    }
  }
  return false;
});

// Methods
const loadSettings = async () => {
  try {
    const savedSettings = await advancedAIStore.loadSettings();
    if (savedSettings) {
      settings.value = { ...settings.value, ...savedSettings };
    }
    originalSettings.value = JSON.stringify(settings.value);
    updateLastSavedTime();
  } catch (error) {
    console.error('Failed to load settings:', error);
  }
};

const saveSettings = async () => {
  try {
    await advancedAIStore.saveSettings(settings.value);
    originalSettings.value = JSON.stringify(settings.value);
    updateLastSavedTime();
  } catch (error) {
    console.error('Failed to save settings:', error);
  }
};

const revertChanges = () => {
  settings.value = JSON.parse(originalSettings.value);
};

const resetToDefaults = () => {
  if (confirm('Are you sure you want to reset all settings to their default values?')) {
    settings.value = {
      general: {
        defaultProseMode: '',
        autoSave: true,
        showAdvancedOptions: false,
        enableStreaming: true
      },
      generation: {
        defaultContextLength: 2000,
        defaultOutputLength: 200,
        defaultCreativity: 0.7,
        enableClicheDetection: true,
        ultraCreativeThreshold: 0.85
      },
      saliency: {
        enabled: true,
        autoBuildContext: true,
        refreshInterval: 'weekly',
        includeRelationships: true,
        includePlotThreads: true
      },
      images: {
        defaultStyle: 'realistic',
        defaultResolution: '768x512',
        defaultQuality: 'standard',
        autoEnhancePrompts: true,
        useStoryContext: true,
        autoSave: true
      },
      brainstorming: {
        defaultDuration: 15,
        ideasPerGeneration: 5,
        autoRate: false,
        useStoryContext: true,
        autoExportKeepers: false
      },
      performance: {
        cacheContent: true,
        preloadModels: false,
        backgroundProcessing: true,
        maxConcurrentGenerations: 2
      },
      privacy: {
        storeHistory: true,
        shareUsageData: false,
        dataRetention: '6months',
        exportFormat: 'json'
      }
    };
  }
};

const exportSettings = () => {
  const dataStr = JSON.stringify(settings.value, null, 2);
  const dataBlob = new Blob([dataStr], { type: 'application/json' });
  const url = URL.createObjectURL(dataBlob);
  
  const link = document.createElement('a');
  link.href = url;
  link.download = `storyweaver-ai-settings-${new Date().toISOString().split('T')[0]}.json`;
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
  
  URL.revokeObjectURL(url);
};

const importSettings = () => {
  showImportModal.value = true;
  importMethod.value = 'file';
  importText.value = '';
};

const handleFileImport = (event: Event) => {
  const file = (event.target as HTMLInputElement).files?.[0];
  if (file) {
    const reader = new FileReader();
    reader.onload = (e) => {
      try {
        const importedSettings = JSON.parse(e.target?.result as string);
        applyImportedSettings(importedSettings);
        showImportModal.value = false;
      } catch (error) {
        alert('Invalid JSON file. Please check the file format.');
      }
    };
    reader.readAsText(file);
  }
};

const processImport = () => {
  if (importMethod.value === 'text') {
    try {
      const importedSettings = JSON.parse(importText.value);
      applyImportedSettings(importedSettings);
      showImportModal.value = false;
    } catch (error) {
      alert('Invalid JSON format. Please check your input.');
    }
  }
};

const applyImportedSettings = (importedSettings: any) => {
  // Validate and merge imported settings
  const validatedSettings = { ...settings.value };
  
  // Safely merge each section
  if (importedSettings.general) {
    validatedSettings.general = { ...validatedSettings.general, ...importedSettings.general };
  }
  if (importedSettings.generation) {
    validatedSettings.generation = { ...validatedSettings.generation, ...importedSettings.generation };
  }
  if (importedSettings.saliency) {
    validatedSettings.saliency = { ...validatedSettings.saliency, ...importedSettings.saliency };
  }
  if (importedSettings.images) {
    validatedSettings.images = { ...validatedSettings.images, ...importedSettings.images };
  }
  if (importedSettings.brainstorming) {
    validatedSettings.brainstorming = { ...validatedSettings.brainstorming, ...importedSettings.brainstorming };
  }
  if (importedSettings.performance) {
    validatedSettings.performance = { ...validatedSettings.performance, ...importedSettings.performance };
  }
  if (importedSettings.privacy) {
    validatedSettings.privacy = { ...validatedSettings.privacy, ...importedSettings.privacy };
  }
  
  settings.value = validatedSettings;
};

const updateLastSavedTime = () => {
  lastSavedTime.value = new Date().toLocaleString();
};

// Auto-save functionality
watch(settings, () => {
  // Debounced auto-save could be implemented here
}, { deep: true });

// Lifecycle
onMounted(() => {
  loadSettings();
  advancedAIStore.loadProseModes();
});
</script>

<style scoped>
.advanced-ai-settings {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-primary);
}

.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid var(--border-color);
  background: var(--bg-secondary);
}

.settings-header h3 {
  margin: 0;
  color: var(--text-primary);
}

.header-actions {
  display: flex;
  gap: 0.5rem;
}

.reset-button,
.export-button,
.import-button {
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

.reset-button:hover,
.export-button:hover,
.import-button:hover {
  background: var(--bg-hover);
}

.settings-content {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
}

.settings-section {
  margin-bottom: 2rem;
  padding: 1.5rem;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
}

.settings-section h4 {
  margin: 0 0 1.5rem 0;
  color: var(--text-primary);
  font-size: 1.125rem;
  border-bottom: 1px solid var(--border-color);
  padding-bottom: 0.5rem;
}

.setting-group {
  margin-bottom: 1.5rem;
}

.setting-group:last-child {
  margin-bottom: 0;
}

.setting-label {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  font-weight: 500;
  color: var(--text-primary);
}

.setting-select,
.setting-checkbox {
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  transition: border-color 0.2s ease;
}

.setting-select:focus {
  outline: none;
  border-color: var(--accent-color);
}

.setting-checkbox {
  width: auto;
  margin-left: auto;
}

.range-input-container {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.setting-range {
  flex: 1;
  height: 6px;
  border-radius: 3px;
  background: var(--border-color);
  outline: none;
  -webkit-appearance: none;
}

.setting-range::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: var(--accent-color);
  cursor: pointer;
}

.setting-range::-moz-range-thumb {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: var(--accent-color);
  cursor: pointer;
  border: none;
}

.range-value {
  min-width: 80px;
  text-align: right;
  font-weight: 500;
  color: var(--text-primary);
  font-size: 0.875rem;
}

.setting-hint {
  font-size: 0.875rem;
  color: var(--text-secondary);
  line-height: 1.4;
  margin-top: 0.25rem;
}

.settings-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-top: 1px solid var(--border-color);
  background: var(--bg-secondary);
}

.footer-info {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.unsaved-indicator {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.875rem;
  color: #ffc107;
}

.unsaved-indicator i {
  font-size: 0.5rem;
}

.last-saved {
  font-size: 0.75rem;
  color: var(--text-secondary);
}

.footer-actions {
  display: flex;
  gap: 0.5rem;
}

.revert-button,
.save-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: transparent;
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.revert-button:hover:not(:disabled) {
  background: var(--bg-hover);
}

.save-button {
  background: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.save-button:hover:not(:disabled) {
  background: var(--accent-color-hover);
}

.revert-button:disabled,
.save-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Modal Styles */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: var(--bg-primary);
  border-radius: 8px;
  width: 90%;
  max-width: 500px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid var(--border-color);
}

.modal-header h3 {
  margin: 0;
  color: var(--text-primary);
}

.close-button {
  padding: 0.5rem;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.2s ease;
}

.close-button:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.modal-body {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
}

.import-options {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.import-option {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
  color: var(--text-primary);
}

.file-input {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
}

.import-textarea {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-family: 'Courier New', monospace;
  resize: vertical;
}

.import-textarea:focus {
  outline: none;
  border-color: var(--accent-color);
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 0.5rem;
  padding: 1.5rem;
  border-top: 1px solid var(--border-color);
}

.cancel-button,
.import-confirm-button {
  padding: 0.75rem 1rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: transparent;
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.cancel-button:hover {
  background: var(--bg-hover);
}

.import-confirm-button {
  background: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.import-confirm-button:hover:not(:disabled) {
  background: var(--accent-color-hover);
}

.import-confirm-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Responsive design */
@media (max-width: 768px) {
  .settings-header {
    flex-direction: column;
    gap: 1rem;
    align-items: stretch;
  }
  
  .header-actions {
    justify-content: center;
  }
  
  .settings-content {
    padding: 1rem;
  }
  
  .settings-section {
    padding: 1rem;
  }
  
  .settings-footer {
    flex-direction: column;
    gap: 1rem;
    align-items: stretch;
  }
  
  .footer-actions {
    justify-content: center;
  }
  
  .range-input-container {
    flex-direction: column;
    align-items: stretch;
  }
  
  .range-value {
    text-align: center;
  }
}
</style>