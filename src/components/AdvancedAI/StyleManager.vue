<template>
  <div class="style-manager">
    <!-- Header -->
    <div class="manager-header">
      <div class="header-content">
        <h3>Style Manager</h3>
        <p class="header-description">
          Manage writing style examples and analyze your prose for consistency and improvement.
        </p>
      </div>
      
      <div class="header-actions">
        <button @click="showAddExample = true" class="add-example-button">
          <i class="fas fa-plus"></i>
          Add Style Example
        </button>
        
        <button @click="analyzeCurrentDocument" :disabled="!canAnalyze" class="analyze-button">
          <i class="fas fa-search"></i>
          Analyze Current Document
        </button>
      </div>
    </div>

    <!-- Tabs -->
    <div class="tab-container">
      <div class="tab-nav">
        <button 
          @click="activeTab = 'examples'"
          :class="['tab-button', { active: activeTab === 'examples' }]"
        >
          <i class="fas fa-book"></i>
          Style Examples
          <span class="tab-count">({{ styleExamples.length }})</span>
        </button>
        
        <button 
          @click="activeTab = 'analysis'"
          :class="['tab-button', { active: activeTab === 'analysis' }]"
        >
          <i class="fas fa-chart-line"></i>
          Style Analysis
          <span v-if="lastAnalysis" class="tab-indicator">•</span>
        </button>
        
        <button 
          @click="activeTab = 'settings'"
          :class="['tab-button', { active: activeTab === 'settings' }]"
        >
          <i class="fas fa-cog"></i>
          Settings
        </button>
      </div>
    </div>

    <!-- Tab Content -->
    <div class="tab-content">
      <!-- Style Examples Tab -->
      <div v-if="activeTab === 'examples'" class="examples-tab">
        <!-- Filter and Sort -->
        <div class="examples-controls">
          <div class="filter-controls">
            <select v-model="filterCategory" class="filter-select">
              <option value="all">All Categories</option>
              <option value="dialogue">Dialogue</option>
              <option value="description">Description</option>
              <option value="action">Action</option>
              <option value="introspection">Introspection</option>
              <option value="narrative">Narrative</option>
              <option value="other">Other</option>
            </select>
            
            <select v-model="sortBy" class="sort-select">
              <option value="newest">Newest First</option>
              <option value="oldest">Oldest First</option>
              <option value="name">Name A-Z</option>
              <option value="category">Category</option>
            </select>
          </div>
          
          <div class="view-controls">
            <button 
              @click="viewMode = 'grid'"
              :class="['view-button', { active: viewMode === 'grid' }]"
            >
              <i class="fas fa-th"></i>
            </button>
            <button 
              @click="viewMode = 'list'"
              :class="['view-button', { active: viewMode === 'list' }]"
            >
              <i class="fas fa-list"></i>
            </button>
          </div>
        </div>

        <!-- Examples Grid/List -->
        <div :class="['examples-container', viewMode]">
          <div 
            v-for="example in filteredAndSortedExamples" 
            :key="example.id"
            class="example-item"
          >
            <!-- Example Header -->
            <div class="example-header">
              <div class="example-title">{{ example.name }}</div>
              <div class="example-actions">
                <button @click="editExample(example)" class="edit-button" title="Edit example">
                  <i class="fas fa-edit"></i>
                </button>
                <button @click="duplicateExample(example)" class="duplicate-button" title="Duplicate example">
                  <i class="fas fa-copy"></i>
                </button>
                <button @click="deleteExample(example)" class="delete-button" title="Delete example">
                  <i class="fas fa-trash"></i>
                </button>
              </div>
            </div>

            <!-- Example Content -->
            <div class="example-content">
              <div class="example-meta">
                <span class="meta-category">{{ formatCategory(example.category) }}</span>
                <span class="meta-date">{{ formatDate(example.created_at) }}</span>
              </div>
              
              <div class="example-text">
                <p>{{ truncateText(example.content, 200) }}</p>
              </div>
              
              <div v-if="example.notes" class="example-notes">
                <div class="notes-header">
                  <i class="fas fa-sticky-note"></i>
                  Notes:
                </div>
                <p class="notes-content">{{ truncateText(example.notes, 100) }}</p>
              </div>
            </div>

            <!-- Example Footer -->
            <div class="example-footer">
              <button @click="viewExample(example)" class="view-button">
                <i class="fas fa-eye"></i>
                View Full
              </button>
              
              <button @click="useAsReference(example)" class="reference-button">
                <i class="fas fa-bookmark"></i>
                Use as Reference
              </button>
            </div>
          </div>
        </div>
        
        <!-- Empty State -->
        <div v-if="filteredAndSortedExamples.length === 0" class="empty-state">
          <div class="empty-icon">
            <i class="fas fa-book"></i>
          </div>
          <h4>No Style Examples</h4>
          <p>{{ getEmptyStateMessage() }}</p>
          <button @click="showAddExample = true" class="add-first-example-button">
            <i class="fas fa-plus"></i>
            Add Your First Style Example
          </button>
        </div>
      </div>

      <!-- Style Analysis Tab -->
      <div v-if="activeTab === 'analysis'" class="analysis-tab">
        <div v-if="!lastAnalysis" class="no-analysis">
          <div class="no-analysis-content">
            <div class="no-analysis-icon">
              <i class="fas fa-chart-line"></i>
            </div>
            <h4>No Analysis Available</h4>
            <p>Analyze a document to see detailed style insights and recommendations.</p>
            <button @click="analyzeCurrentDocument" :disabled="!canAnalyze" class="analyze-now-button">
              <i class="fas fa-search"></i>
              Analyze Current Document
            </button>
          </div>
        </div>
        
        <div v-else class="analysis-content">
          <!-- Analysis Header -->
          <div class="analysis-header">
            <div class="analysis-info">
              <h4>Style Analysis Results</h4>
              <div class="analysis-meta">
                <span class="meta-item">
                  <i class="fas fa-file-alt"></i>
                  {{ lastAnalysis.document_name }}
                </span>
                <span class="meta-item">
                  <i class="fas fa-clock"></i>
                  {{ formatDateTime(lastAnalysis.analyzed_at) }}
                </span>
                <span class="meta-item">
                  <i class="fas fa-chart-bar"></i>
                  {{ lastAnalysis.word_count }} words
                </span>
              </div>
            </div>
            
            <div class="analysis-actions">
              <button @click="exportAnalysis" class="export-button">
                <i class="fas fa-download"></i>
                Export
              </button>
              
              <button @click="analyzeCurrentDocument" :disabled="!canAnalyze" class="reanalyze-button">
                <i class="fas fa-redo"></i>
                Re-analyze
              </button>
            </div>
          </div>

          <!-- Analysis Metrics -->
          <div class="analysis-metrics">
            <div class="metric-card">
              <div class="metric-header">
                <i class="fas fa-tachometer-alt"></i>
                <span>Readability</span>
              </div>
              <div class="metric-value">{{ lastAnalysis.readability_score }}/100</div>
              <div class="metric-description">{{ getReadabilityDescription(lastAnalysis.readability_score) }}</div>
            </div>
            
            <div class="metric-card">
              <div class="metric-header">
                <i class="fas fa-balance-scale"></i>
                <span>Sentence Variety</span>
              </div>
              <div class="metric-value">{{ lastAnalysis.sentence_variety_score }}/100</div>
              <div class="metric-description">{{ getSentenceVarietyDescription(lastAnalysis.sentence_variety_score) }}</div>
            </div>
            
            <div class="metric-card">
              <div class="metric-header">
                <i class="fas fa-palette"></i>
                <span>Vocabulary Richness</span>
              </div>
              <div class="metric-value">{{ lastAnalysis.vocabulary_richness }}/100</div>
              <div class="metric-description">{{ getVocabularyDescription(lastAnalysis.vocabulary_richness) }}</div>
            </div>
            
            <div class="metric-card">
              <div class="metric-header">
                <i class="fas fa-heartbeat"></i>
                <span>Pacing</span>
              </div>
              <div class="metric-value">{{ lastAnalysis.pacing_score }}/100</div>
              <div class="metric-description">{{ getPacingDescription(lastAnalysis.pacing_score) }}</div>
            </div>
          </div>

          <!-- Style Characteristics -->
          <div class="style-characteristics">
            <h5>Style Characteristics</h5>
            <div class="characteristics-grid">
              <div 
                v-for="characteristic in lastAnalysis.style_characteristics" 
                :key="characteristic.name"
                class="characteristic-item"
              >
                <div class="characteristic-name">{{ characteristic.name }}</div>
                <div class="characteristic-bar">
                  <div 
                    class="characteristic-fill"
                    :style="{ width: `${characteristic.score}%` }"
                  ></div>
                </div>
                <div class="characteristic-score">{{ characteristic.score }}%</div>
              </div>
            </div>
          </div>

          <!-- Recommendations -->
          <div class="recommendations">
            <h5>Recommendations</h5>
            <div class="recommendations-list">
              <div 
                v-for="recommendation in lastAnalysis.recommendations" 
                :key="recommendation.id"
                :class="['recommendation-item', recommendation.priority]"
              >
                <div class="recommendation-header">
                  <i :class="getRecommendationIcon(recommendation.type)"></i>
                  <span class="recommendation-title">{{ recommendation.title }}</span>
                  <span class="recommendation-priority">{{ recommendation.priority }}</span>
                </div>
                <p class="recommendation-description">{{ recommendation.description }}</p>
                <div v-if="recommendation.examples.length > 0" class="recommendation-examples">
                  <div class="examples-header">Examples:</div>
                  <ul class="examples-list">
                    <li v-for="example in recommendation.examples" :key="example">
                      {{ example }}
                    </li>
                  </ul>
                </div>
              </div>
            </div>
          </div>

          <!-- Comparison with Examples -->
          <div v-if="lastAnalysis.style_comparisons.length > 0" class="style-comparisons">
            <h5>Comparison with Style Examples</h5>
            <div class="comparisons-list">
              <div 
                v-for="comparison in lastAnalysis.style_comparisons" 
                :key="comparison.example_id"
                class="comparison-item"
              >
                <div class="comparison-header">
                  <span class="comparison-name">{{ comparison.example_name }}</span>
                  <span class="comparison-similarity">{{ comparison.similarity_score }}% similar</span>
                </div>
                <div class="comparison-details">
                  <div class="detail-item">
                    <span class="detail-label">Tone:</span>
                    <span class="detail-value">{{ comparison.tone_match }}% match</span>
                  </div>
                  <div class="detail-item">
                    <span class="detail-label">Complexity:</span>
                    <span class="detail-value">{{ comparison.complexity_match }}% match</span>
                  </div>
                  <div class="detail-item">
                    <span class="detail-label">Rhythm:</span>
                    <span class="detail-value">{{ comparison.rhythm_match }}% match</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Settings Tab -->
      <div v-if="activeTab === 'settings'" class="settings-tab">
        <div class="settings-content">
          <div class="settings-section">
            <h4>Analysis Settings</h4>
            
            <div class="setting-group">
              <label class="setting-label">
                <input type="checkbox" v-model="analysisSettings.includeDialogue">
                Include dialogue in analysis
              </label>
              <p class="setting-description">
                Whether to include dialogue when analyzing writing style.
              </p>
            </div>
            
            <div class="setting-group">
              <label class="setting-label">
                <input type="checkbox" v-model="analysisSettings.detectCliches">
                Detect clichés and overused phrases
              </label>
              <p class="setting-description">
                Identify potentially overused expressions and suggest alternatives.
              </p>
            </div>
            
            <div class="setting-group">
              <label class="setting-label">
                <input type="checkbox" v-model="analysisSettings.analyzeEmotionalTone">
                Analyze emotional tone
              </label>
              <p class="setting-description">
                Assess the emotional impact and tone of the writing.
              </p>
            </div>
            
            <div class="setting-group">
              <label class="setting-label">
                <input type="checkbox" v-model="analysisSettings.compareWithExamples">
                Compare with style examples
              </label>
              <p class="setting-description">
                Compare analyzed text with your saved style examples.
              </p>
            </div>
          </div>
          
          <div class="settings-section">
            <h4>Style Example Settings</h4>
            
            <div class="setting-group">
              <label for="default-category">Default Category:</label>
              <select id="default-category" v-model="exampleSettings.defaultCategory" class="setting-select">
                <option value="dialogue">Dialogue</option>
                <option value="description">Description</option>
                <option value="action">Action</option>
                <option value="introspection">Introspection</option>
                <option value="narrative">Narrative</option>
                <option value="other">Other</option>
              </select>
            </div>
            
            <div class="setting-group">
              <label for="auto-categorize">Auto-categorize examples:</label>
              <select id="auto-categorize" v-model="exampleSettings.autoCategorize" class="setting-select">
                <option value="never">Never</option>
                <option value="suggest">Suggest category</option>
                <option value="always">Always auto-categorize</option>
              </select>
            </div>
          </div>
          
          <div class="settings-section">
            <h4>Export Settings</h4>
            
            <div class="setting-group">
              <label for="export-format">Default Export Format:</label>
              <select id="export-format" v-model="exportSettings.defaultFormat" class="setting-select">
                <option value="markdown">Markdown</option>
                <option value="html">HTML</option>
                <option value="pdf">PDF</option>
                <option value="json">JSON</option>
              </select>
            </div>
            
            <div class="setting-group">
              <label class="setting-label">
                <input type="checkbox" v-model="exportSettings.includeMetadata">
                Include metadata in exports
              </label>
            </div>
          </div>
          
          <div class="settings-actions">
            <button @click="saveSettings" class="save-settings-button">
              <i class="fas fa-save"></i>
              Save Settings
            </button>
            
            <button @click="resetSettings" class="reset-settings-button">
              <i class="fas fa-undo"></i>
              Reset to Defaults
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Add/Edit Example Modal -->
    <StyleExampleModal 
      v-if="showAddExample || editingExample"
      :example="editingExample"
      @close="closeExampleModal"
      @save="saveExample"
    />

    <!-- Example Detail Modal -->
    <ExampleDetailModal 
      v-if="viewingExample"
      :example="viewingExample"
      @close="viewingExample = null"
      @edit="editExample"
      @delete="deleteExample"
    />

    <!-- Analysis Progress -->
    <div v-if="isAnalyzing" class="analysis-progress">
      <div class="progress-content">
        <div class="progress-spinner"></div>
        <div class="progress-text">
          <h4>Analyzing Style...</h4>
          <p>AI is analyzing your writing style and generating insights.</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useAdvancedAIStore } from '../../stores/advancedAIStore';
import { useProjectStore } from '../../stores/projectStore';
import { useDocumentStore } from '../../stores/documentStore';
import StyleExampleModal from './StyleExampleModal.vue';
import ExampleDetailModal from './ExampleDetailModal.vue';
import type { StyleExample, StyleAnalysis } from '../../types/advancedAI';

// Stores
const advancedAIStore = useAdvancedAIStore();
const projectStore = useProjectStore();
const documentStore = useDocumentStore();

// Reactive state
const activeTab = ref<'examples' | 'analysis' | 'settings'>('examples');
const filterCategory = ref('all');
const sortBy = ref('newest');
const viewMode = ref<'grid' | 'list'>('grid');
const showAddExample = ref(false);
const editingExample = ref<StyleExample | null>(null);
const viewingExample = ref<StyleExample | null>(null);

// Settings
const analysisSettings = ref({
  includeDialogue: true,
  detectCliches: true,
  analyzeEmotionalTone: true,
  compareWithExamples: true
});

const exampleSettings = ref({
  defaultCategory: 'narrative',
  autoCategorize: 'suggest'
});

const exportSettings = ref({
  defaultFormat: 'markdown',
  includeMetadata: true
});

// Computed properties
const styleExamples = computed(() => advancedAIStore.styleExamples);
const lastAnalysis = computed(() => advancedAIStore.lastStyleAnalysis);
const isAnalyzing = computed(() => advancedAIStore.isAnalyzingStyle);

const canAnalyze = computed(() => {
  return documentStore.currentDocument && 
         documentStore.currentDocument.content && 
         documentStore.currentDocument.content.trim().length > 0;
});

const filteredAndSortedExamples = computed(() => {
  let examples = [...styleExamples.value];
  
  // Filter by category
  if (filterCategory.value !== 'all') {
    examples = examples.filter(example => example.category === filterCategory.value);
  }
  
  // Sort
  switch (sortBy.value) {
    case 'newest':
      examples.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime());
      break;
    case 'oldest':
      examples.sort((a, b) => new Date(a.created_at).getTime() - new Date(b.created_at).getTime());
      break;
    case 'name':
      examples.sort((a, b) => a.name.localeCompare(b.name));
      break;
    case 'category':
      examples.sort((a, b) => a.category.localeCompare(b.category));
      break;
  }
  
  return examples;
});

// Methods
const editExample = (example: StyleExample) => {
  editingExample.value = example;
  showAddExample.value = true;
};

const duplicateExample = (example: StyleExample) => {
  const duplicated = {
    ...example,
    id: `${example.id}_copy_${Date.now()}`,
    name: `${example.name} (Copy)`,
    created_at: new Date().toISOString()
  };
  editingExample.value = duplicated;
  showAddExample.value = true;
};

const deleteExample = async (example: StyleExample) => {
  if (confirm(`Are you sure you want to delete "${example.name}"?`)) {
    try {
      await advancedAIStore.deleteStyleExample(example.id);
    } catch (error) {
      console.error('Failed to delete style example:', error);
    }
  }
};

const viewExample = (example: StyleExample) => {
  viewingExample.value = example;
};

const useAsReference = (example: StyleExample) => {
  // This would set the example as a reference for generation
  advancedAIStore.setActiveStyleReference(example.id);
};

const closeExampleModal = () => {
  showAddExample.value = false;
  editingExample.value = null;
};

const saveExample = async (example: StyleExample) => {
  try {
    if (editingExample.value) {
      await advancedAIStore.updateStyleExample(example);
    } else {
      await advancedAIStore.createStyleExample(example);
    }
    closeExampleModal();
  } catch (error) {
    console.error('Failed to save style example:', error);
  }
};

const analyzeCurrentDocument = async () => {
  if (!canAnalyze.value || !documentStore.currentDocument) return;
  
  try {
    await advancedAIStore.analyzeDocumentStyle({
      document_id: documentStore.currentDocument.id,
      content: documentStore.currentDocument.content,
      settings: analysisSettings.value
    });
    activeTab.value = 'analysis';
  } catch (error) {
    console.error('Failed to analyze document style:', error);
  }
};

const exportAnalysis = () => {
  if (!lastAnalysis.value) return;
  
  const analysisData = {
    document_name: lastAnalysis.value.document_name,
    analyzed_at: lastAnalysis.value.analyzed_at,
    metrics: {
      readability_score: lastAnalysis.value.readability_score,
      sentence_variety_score: lastAnalysis.value.sentence_variety_score,
      vocabulary_richness: lastAnalysis.value.vocabulary_richness,
      pacing_score: lastAnalysis.value.pacing_score
    },
    characteristics: lastAnalysis.value.style_characteristics,
    recommendations: lastAnalysis.value.recommendations,
    comparisons: lastAnalysis.value.style_comparisons
  };
  
  const blob = new Blob([JSON.stringify(analysisData, null, 2)], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = `style_analysis_${lastAnalysis.value.document_name}_${new Date().toISOString().split('T')[0]}.json`;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
};

const saveSettings = () => {
  // Save settings to store or local storage
  localStorage.setItem('styleManagerSettings', JSON.stringify({
    analysis: analysisSettings.value,
    examples: exampleSettings.value,
    export: exportSettings.value
  }));
};

const resetSettings = () => {
  analysisSettings.value = {
    includeDialogue: true,
    detectCliches: true,
    analyzeEmotionalTone: true,
    compareWithExamples: true
  };
  
  exampleSettings.value = {
    defaultCategory: 'narrative',
    autoCategorize: 'suggest'
  };
  
  exportSettings.value = {
    defaultFormat: 'markdown',
    includeMetadata: true
  };
};

// Utility functions
const formatCategory = (category: string): string => {
  return category.charAt(0).toUpperCase() + category.slice(1);
};

const formatDate = (dateString: string): string => {
  const date = new Date(dateString);
  return date.toLocaleDateString();
};

const formatDateTime = (dateString: string): string => {
  const date = new Date(dateString);
  return date.toLocaleString();
};

const truncateText = (text: string, maxLength: number): string => {
  if (text.length <= maxLength) return text;
  return text.substring(0, maxLength) + '...';
};

const getEmptyStateMessage = (): string => {
  if (filterCategory.value !== 'all') {
    return `No ${formatCategory(filterCategory.value)} examples found. Try changing the filter or add a new example.`;
  }
  return 'Add style examples to analyze and maintain consistency in your writing.';
};

const getReadabilityDescription = (score: number): string => {
  if (score >= 80) return 'Very Easy';
  if (score >= 60) return 'Easy';
  if (score >= 40) return 'Moderate';
  if (score >= 20) return 'Difficult';
  return 'Very Difficult';
};

const getSentenceVarietyDescription = (score: number): string => {
  if (score >= 80) return 'Excellent variety';
  if (score >= 60) return 'Good variety';
  if (score >= 40) return 'Moderate variety';
  return 'Limited variety';
};

const getVocabularyDescription = (score: number): string => {
  if (score >= 80) return 'Rich vocabulary';
  if (score >= 60) return 'Good vocabulary';
  if (score >= 40) return 'Adequate vocabulary';
  return 'Limited vocabulary';
};

const getPacingDescription = (score: number): string => {
  if (score >= 80) return 'Well-paced';
  if (score >= 60) return 'Good pacing';
  if (score >= 40) return 'Uneven pacing';
  return 'Poor pacing';
};

const getRecommendationIcon = (type: string): string => {
  switch (type) {
    case 'readability': return 'fas fa-eye';
    case 'variety': return 'fas fa-random';
    case 'vocabulary': return 'fas fa-book';
    case 'pacing': return 'fas fa-tachometer-alt';
    case 'structure': return 'fas fa-sitemap';
    default: return 'fas fa-lightbulb';
  }
};

// Lifecycle
onMounted(async () => {
  if (projectStore.currentProject) {
    await advancedAIStore.loadStyleExamples(projectStore.currentProject.id);
  }
  
  // Load saved settings
  const savedSettings = localStorage.getItem('styleManagerSettings');
  if (savedSettings) {
    try {
      const settings = JSON.parse(savedSettings);
      if (settings.analysis) analysisSettings.value = settings.analysis;
      if (settings.examples) exampleSettings.value = settings.examples;
      if (settings.export) exportSettings.value = settings.export;
    } catch (error) {
      console.error('Failed to load saved settings:', error);
    }
  }
});
</script>

<style scoped>
.style-manager {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  height: 100%;
}

.manager-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 1rem;
  padding: 1.5rem;
  background: var(--bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.header-content h3 {
  margin: 0 0 0.5rem 0;
  color: var(--text-primary);
}

.header-description {
  margin: 0;
  color: var(--text-secondary);
  font-size: 0.875rem;
}

.header-actions {
  display: flex;
  gap: 0.5rem;
}

.add-example-button,
.analyze-button {
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

.add-example-button {
  background: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.add-example-button:hover {
  background: var(--accent-color-hover);
}

.analyze-button:hover:not(:disabled) {
  background: var(--bg-hover);
}

.analyze-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.tab-container {
  border-bottom: 1px solid var(--border-color);
}

.tab-nav {
  display: flex;
  gap: 0.5rem;
}

.tab-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1rem;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  border-bottom: 2px solid transparent;
  transition: all 0.2s ease;
}

.tab-button:hover,
.tab-button.active {
  color: var(--accent-color);
  border-bottom-color: var(--accent-color);
}

.tab-count {
  font-size: 0.75rem;
  opacity: 0.7;
}

.tab-indicator {
  color: var(--accent-color);
  font-size: 0.75rem;
}

.tab-content {
  flex: 1;
  overflow: hidden;
}

.examples-tab,
.analysis-tab,
.settings-tab {
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.examples-controls {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1rem;
}

.filter-controls {
  display: flex;
  gap: 0.5rem;
}

.filter-select,
.sort-select {
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
}

.view-controls {
  display: flex;
  gap: 0.5rem;
}

.view-button {
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.view-button:hover,
.view-button.active {
  background: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.examples-container {
  flex: 1;
  overflow-y: auto;
}

.examples-container.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: 1rem;
}

.examples-container.list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.example-item {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 1rem;
  transition: all 0.2s ease;
}

.example-item:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.example-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 1rem;
  margin-bottom: 0.75rem;
}

.example-title {
  font-weight: 600;
  color: var(--text-primary);
  flex: 1;
}

.example-actions {
  display: flex;
  gap: 0.25rem;
}

.edit-button,
.duplicate-button,
.delete-button {
  padding: 0.25rem;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.2s ease;
}

.edit-button:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.duplicate-button:hover {
  background: var(--bg-hover);
  color: var(--accent-color);
}

.delete-button:hover {
  background: var(--bg-hover);
  color: #dc3545;
}

.example-content {
  margin-bottom: 0.75rem;
}

.example-meta {
  display: flex;
  gap: 1rem;
  margin-bottom: 0.5rem;
  font-size: 0.875rem;
}

.meta-category {
  padding: 0.125rem 0.5rem;
  background: var(--accent-color);
  color: white;
  border-radius: 12px;
  font-size: 0.75rem;
}

.meta-date {
  color: var(--text-secondary);
}

.example-text {
  margin-bottom: 0.75rem;
}

.example-text p {
  margin: 0;
  color: var(--text-primary);
  line-height: 1.5;
}

.example-notes {
  margin-bottom: 0.75rem;
}

.notes-header {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--text-secondary);
  margin-bottom: 0.25rem;
}

.notes-content {
  margin: 0;
  padding: 0.5rem;
  background: var(--bg-primary);
  border-radius: 4px;
  font-size: 0.875rem;
  color: var(--text-primary);
  border: 1px solid var(--border-color);
}

.example-footer {
  display: flex;
  gap: 0.5rem;
  padding-top: 0.75rem;
  border-top: 1px solid var(--border-color);
}

.view-button,
.reference-button {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.25rem 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 0.75rem;
  transition: all 0.2s ease;
}

.view-button:hover,
.reference-button:hover {
  background: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem;
  text-align: center;
  color: var(--text-secondary);
}

.empty-icon {
  font-size: 3rem;
  margin-bottom: 1rem;
  opacity: 0.5;
}

.empty-state h4 {
  margin: 0 0 0.5rem 0;
  color: var(--text-primary);
}

.empty-state p {
  margin: 0 0 1rem 0;
}

.add-first-example-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  border: 1px solid var(--accent-color);
  border-radius: 4px;
  background: var(--accent-color);
  color: white;
  cursor: pointer;
  transition: all 0.2s ease;
}

.add-first-example-button:hover {
  background: var(--accent-color-hover);
}

/* Analysis Tab Styles */
.no-analysis {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
}

.no-analysis-content {
  text-align: center;
  color: var(--text-secondary);
}

.no-analysis-icon {
  font-size: 3rem;
  margin-bottom: 1rem;
  opacity: 0.5;
}

.no-analysis-content h4 {
  margin: 0 0 0.5rem 0;
  color: var(--text-primary);
}

.no-analysis-content p {
  margin: 0 0 1rem 0;
}

.analyze-now-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  border: 1px solid var(--accent-color);
  border-radius: 4px;
  background: var(--accent-color);
  color: white;
  cursor: pointer;
  transition: all 0.2s ease;
}

.analyze-now-button:hover:not(:disabled) {
  background: var(--accent-color-hover);
}

.analyze-now-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.analysis-content {
  flex: 1;
  overflow-y: auto;
}

.analysis-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 1rem;
  margin-bottom: 1.5rem;
  padding: 1rem;
  background: var(--bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.analysis-info h4 {
  margin: 0 0 0.5rem 0;
  color: var(--text-primary);
}

.analysis-meta {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
}

.meta-item {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  font-size: 0.875rem;
  color: var(--text-secondary);
}

.analysis-actions {
  display: flex;
  gap: 0.5rem;
}

.export-button,
.reanalyze-button {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.5rem 1rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: transparent;
  color: var(--text-primary);
  cursor: pointer;
  font-size: 0.875rem;
  transition: all 0.2s ease;
}

.export-button:hover,
.reanalyze-button:hover:not(:disabled) {
  background: var(--bg-hover);
}

.reanalyze-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.analysis-metrics {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 2rem;
}

.metric-card {
  padding: 1rem;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  text-align: center;
}

.metric-header {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
  color: var(--text-secondary);
  font-size: 0.875rem;
}

.metric-value {
  font-size: 2rem;
  font-weight: bold;
  color: var(--accent-color);
  margin-bottom: 0.25rem;
}

.metric-description {
  font-size: 0.875rem;
  color: var(--text-secondary);
}

.style-characteristics,
.recommendations,
.style-comparisons {
  margin-bottom: 2rem;
}

.style-characteristics h5,
.recommendations h5,
.style-comparisons h5 {
  margin: 0 0 1rem 0;
  color: var(--text-primary);
  font-size: 1.125rem;
}

.characteristics-grid {
  display: grid;
  gap: 0.75rem;
}

.characteristic-item {
  display: grid;
  grid-template-columns: 1fr 2fr auto;
  gap: 1rem;
  align-items: center;
  padding: 0.75rem;
  background: var(--bg-secondary);
  border-radius: 6px;
  border: 1px solid var(--border-color);
}

.characteristic-name {
  font-weight: 500;
  color: var(--text-primary);
}

.characteristic-bar {
  height: 8px;
  background: var(--border-color);
  border-radius: 4px;
  overflow: hidden;
}

.characteristic-fill {
  height: 100%;
  background: var(--accent-color);
  transition: width 0.3s ease;
}

.characteristic-score {
  font-weight: 500;
  color: var(--text-secondary);
  font-size: 0.875rem;
}

.recommendations-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.recommendation-item {
  padding: 1rem;
  background: var(--bg-secondary);
  border-radius: 8px;
  border-left: 4px solid var(--border-color);
}

.recommendation-item.high {
  border-left-color: #dc3545;
}

.recommendation-item.medium {
  border-left-color: #ffc107;
}

.recommendation-item.low {
  border-left-color: #28a745;
}

.recommendation-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
}

.recommendation-title {
  font-weight: 500;
  color: var(--text-primary);
  flex: 1;
}

.recommendation-priority {
  padding: 0.125rem 0.5rem;
  border-radius: 12px;
  font-size: 0.75rem;
  font-weight: 500;
  text-transform: uppercase;
}

.recommendation-item.high .recommendation-priority {
  background: #dc3545;
  color: white;
}

.recommendation-item.medium .recommendation-priority {
  background: #ffc107;
  color: #212529;
}

.recommendation-item.low .recommendation-priority {
  background: #28a745;
  color: white;
}

.recommendation-description {
  margin: 0 0 0.75rem 0;
  color: var(--text-primary);
  line-height: 1.5;
}

.recommendation-examples {
  margin-top: 0.75rem;
}

.examples-header {
  font-weight: 500;
  color: var(--text-secondary);
  margin-bottom: 0.25rem;
  font-size: 0.875rem;
}

.examples-list {
  margin: 0;
  padding-left: 1.5rem;
  color: var(--text-primary);
}

.examples-list li {
  margin-bottom: 0.25rem;
  font-size: 0.875rem;
}

.comparisons-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.comparison-item {
  padding: 1rem;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
}

.comparison-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.75rem;
}

.comparison-name {
  font-weight: 500;
  color: var(--text-primary);
}

.comparison-similarity {
  font-weight: 500;
  color: var(--accent-color);
}

.comparison-details {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 0.5rem;
}

.detail-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.25rem 0;
  font-size: 0.875rem;
}

.detail-label {
  color: var(--text-secondary);
}

.detail-value {
  color: var(--text-primary);
  font-weight: 500;
}

/* Settings Tab Styles */
.settings-content {
  max-width: 600px;
}

.settings-section {
  margin-bottom: 2rem;
  padding: 1.5rem;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
}

.settings-section h4 {
  margin: 0 0 1rem 0;
  color: var(--text-primary);
}

.setting-group {
  margin-bottom: 1rem;
}

.setting-group:last-child {
  margin-bottom: 0;
}

.setting-label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
  color: var(--text-primary);
  font-weight: 500;
}

.setting-label input[type="checkbox"] {
  margin: 0;
}

.setting-description {
  margin: 0.25rem 0 0 1.5rem;
  font-size: 0.875rem;
  color: var(--text-secondary);
}

.setting-select {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  margin-top: 0.25rem;
}

.settings-actions {
  display: flex;
  gap: 0.5rem;
  margin-top: 2rem;
}

.save-settings-button,
.reset-settings-button {
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

.save-settings-button {
  background: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.save-settings-button:hover {
  background: var(--accent-color-hover);
}

.reset-settings-button:hover {
  background: var(--bg-hover);
}

.analysis-progress {
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

.progress-content {
  background: var(--bg-primary);
  padding: 2rem;
  border-radius: 8px;
  text-align: center;
  max-width: 400px;
}

.progress-spinner {
  width: 40px;
  height: 40px;
  border: 4px solid var(--border-color);
  border-top: 4px solid var(--accent-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 1rem;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.progress-text h4 {
  margin: 0 0 0.5rem 0;
  color: var(--text-primary);
}

.progress-text p {
  margin: 0;
  color: var(--text-secondary);
  font-size: 0.875rem;
}

/* Responsive design */
@media (max-width: 768px) {
  .manager-header {
    flex-direction: column;
    align-items: stretch;
  }
  
  .header-actions {
    flex-direction: column;
  }
  
  .examples-controls {
    flex-direction: column;
    align-items: stretch;
  }
  
  .filter-controls {
    justify-content: space-between;
  }
  
  .examples-container.grid {
    grid-template-columns: 1fr;
  }
  
  .analysis-header {
    flex-direction: column;
  }
  
  .analysis-metrics {
    grid-template-columns: 1fr;
  }
  
  .characteristic-item {
    grid-template-columns: 1fr;
    gap: 0.5rem;
    text-align: center;
  }
  
  .comparison-details {
    grid-template-columns: 1fr;
  }
  
  .settings-actions {
    flex-direction: column;
  }
}
</style>