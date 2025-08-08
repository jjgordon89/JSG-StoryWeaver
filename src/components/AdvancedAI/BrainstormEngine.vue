<template>
  <div class="brainstorm-engine">
    <!-- Session Header -->
    <div class="session-header">
      <div class="header-content">
        <h3>Brainstorming Engine</h3>
        <p class="header-description">
          Generate and organize creative ideas for your story using AI-powered brainstorming.
        </p>
      </div>
      
      <div class="session-controls">
        <select v-model="selectedSessionId" @change="loadSession" class="session-select">
          <option value="">New Session</option>
          <option 
            v-for="session in sessions" 
            :key="session.id"
            :value="session.id"
          >
            {{ session.title }} ({{ session.ideas.length }} ideas)
          </option>
        </select>
        
        <button @click="createNewSession" class="new-session-button">
          <i class="fas fa-plus"></i>
          New Session
        </button>
      </div>
    </div>

    <!-- Session Configuration -->
    <div v-if="!currentSession" class="session-config">
      <div class="config-form">
        <h4>Start New Brainstorming Session</h4>
        
        <div class="form-group">
          <label for="session-title">Session Title:</label>
          <input 
            id="session-title"
            v-model="newSessionTitle"
            type="text"
            placeholder="e.g., Character Development, Plot Twists, World Building"
            class="form-input"
          >
        </div>
        
        <div class="form-group">
          <label for="session-category">Category:</label>
          <select id="session-category" v-model="newSessionCategory" class="form-select">
            <option value="characters">Characters</option>
            <option value="plot">Plot & Story</option>
            <option value="worldbuilding">World Building</option>
            <option value="themes">Themes & Concepts</option>
            <option value="dialogue">Dialogue & Voice</option>
            <option value="scenes">Scenes & Settings</option>
            <option value="conflicts">Conflicts & Tension</option>
            <option value="general">General Ideas</option>
          </select>
        </div>
        
        <div class="form-group">
          <label for="session-context">Context & Focus:</label>
          <textarea 
            id="session-context"
            v-model="newSessionContext"
            placeholder="Describe what you want to brainstorm about. Include any relevant story details, constraints, or specific areas you want to explore..."
            rows="3"
            class="form-textarea"
          ></textarea>
        </div>
        
        <div class="form-actions">
          <button @click="startSession" :disabled="!canStartSession" class="start-button">
            <i class="fas fa-lightbulb"></i>
            Start Brainstorming
          </button>
        </div>
      </div>
    </div>

    <!-- Active Session -->
    <div v-else class="active-session">
      <!-- Session Info -->
      <div class="session-info">
        <div class="session-details">
          <h4>{{ currentSession.title }}</h4>
          <div class="session-meta">
            <span class="meta-item">
              <i class="fas fa-tag"></i>
              {{ formatCategory(currentSession.category) }}
            </span>
            <span class="meta-item">
              <i class="fas fa-lightbulb"></i>
              {{ currentSession.ideas.length }} ideas
            </span>
            <span class="meta-item">
              <i class="fas fa-star"></i>
              {{ keeperCount }} keepers
            </span>
            <span class="meta-item">
              <i class="fas fa-clock"></i>
              {{ formatDate(currentSession.created_at) }}
            </span>
          </div>
        </div>
        
        <div class="session-actions">
          <button @click="generateIdeas" :disabled="isGenerating" class="generate-button">
            <i class="fas fa-magic"></i>
            {{ isGenerating ? 'Generating...' : 'Generate Ideas' }}
          </button>
          
          <button @click="exportKeepers" :disabled="keeperCount === 0" class="export-button">
            <i class="fas fa-download"></i>
            Export Keepers
          </button>
          
          <button @click="saveSession" class="save-button">
            <i class="fas fa-save"></i>
            Save
          </button>
        </div>
      </div>

      <!-- Generation Settings -->
      <div class="generation-settings" v-if="showSettings">
        <div class="settings-grid">
          <div class="setting-group">
            <label>Creativity Level:</label>
            <div class="slider-container">
              <input 
                type="range" 
                v-model="creativityLevel" 
                min="1" 
                max="10" 
                class="creativity-slider"
              >
              <div class="slider-labels">
                <span>Conservative</span>
                <span>{{ creativityLevel }}</span>
                <span>Wild</span>
              </div>
            </div>
          </div>
          
          <div class="setting-group">
            <label>Ideas per Generation:</label>
            <select v-model="ideasPerGeneration" class="setting-select">
              <option value="3">3 ideas</option>
              <option value="5">5 ideas</option>
              <option value="8">8 ideas</option>
              <option value="10">10 ideas</option>
            </select>
          </div>
          
          <div class="setting-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="useStoryContext">
              Use story context
            </label>
          </div>
          
          <div class="setting-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="buildOnExisting">
              Build on existing ideas
            </label>
          </div>
        </div>
      </div>
      
      <button @click="showSettings = !showSettings" class="settings-toggle">
        <i class="fas fa-cog"></i>
        {{ showSettings ? 'Hide' : 'Show' }} Settings
      </button>

      <!-- Ideas Display -->
      <div class="ideas-section">
        <!-- Filter and Sort -->
        <div class="ideas-controls">
          <div class="filter-controls">
            <select v-model="filterBy" class="filter-select">
              <option value="all">All Ideas</option>
              <option value="keepers">Keepers Only</option>
              <option value="rated">Rated Ideas</option>
              <option value="unrated">Unrated Ideas</option>
            </select>
            
            <select v-model="sortBy" class="sort-select">
              <option value="newest">Newest First</option>
              <option value="oldest">Oldest First</option>
              <option value="rating">Highest Rated</option>
              <option value="alphabetical">Alphabetical</option>
            </select>
          </div>
          
          <div class="view-controls">
            <button 
              @click="viewMode = 'cards'" 
              :class="{ active: viewMode === 'cards' }"
              class="view-button"
            >
              <i class="fas fa-th"></i>
            </button>
            <button 
              @click="viewMode = 'list'" 
              :class="{ active: viewMode === 'list' }"
              class="view-button"
            >
              <i class="fas fa-list"></i>
            </button>
          </div>
        </div>

        <!-- Ideas Grid/List -->
        <div :class="['ideas-container', viewMode]">
          <div 
            v-for="idea in filteredAndSortedIdeas" 
            :key="idea.id"
            :class="['idea-item', { keeper: idea.is_keeper, rated: idea.rating > 0 }]"
          >
            <!-- Idea Header -->
            <div class="idea-header">
              <div class="idea-title">{{ idea.title }}</div>
              <div class="idea-actions">
                <button 
                  @click="toggleKeeper(idea)"
                  :class="['keeper-button', { active: idea.is_keeper }]"
                  :title="idea.is_keeper ? 'Remove from keepers' : 'Mark as keeper'"
                >
                  <i class="fas fa-star"></i>
                </button>
                
                <div class="rating-controls">
                  <button 
                    v-for="rating in 5" 
                    :key="rating"
                    @click="rateIdea(idea, rating)"
                    :class="['rating-star', { active: rating <= idea.rating }]"
                  >
                    <i class="fas fa-star"></i>
                  </button>
                </div>
                
                <button @click="deleteIdea(idea)" class="delete-button" title="Delete idea">
                  <i class="fas fa-trash"></i>
                </button>
              </div>
            </div>

            <!-- Idea Content -->
            <div class="idea-content">
              <p class="idea-description">{{ idea.description }}</p>
              
              <!-- Tags -->
              <div v-if="idea.tags.length > 0" class="idea-tags">
                <span 
                  v-for="tag in idea.tags" 
                  :key="tag"
                  class="idea-tag"
                >
                  {{ tag }}
                </span>
              </div>
              
              <!-- Notes -->
              <div v-if="idea.notes" class="idea-notes">
                <div class="notes-header">
                  <i class="fas fa-sticky-note"></i>
                  Notes:
                </div>
                <p class="notes-content">{{ idea.notes }}</p>
              </div>
              
              <!-- Add/Edit Notes -->
              <div class="notes-editor" v-if="editingNotes === idea.id">
                <textarea 
                  v-model="noteText"
                  placeholder="Add your notes about this idea..."
                  rows="3"
                  class="notes-textarea"
                ></textarea>
                <div class="notes-actions">
                  <button @click="saveNotes(idea)" class="save-notes-button">
                    <i class="fas fa-check"></i>
                    Save
                  </button>
                  <button @click="cancelNotes" class="cancel-notes-button">
                    <i class="fas fa-times"></i>
                    Cancel
                  </button>
                </div>
              </div>
              
              <button 
                v-else
                @click="startEditingNotes(idea)"
                class="add-notes-button"
              >
                <i class="fas fa-plus"></i>
                {{ idea.notes ? 'Edit Notes' : 'Add Notes' }}
              </button>
            </div>

            <!-- Idea Footer -->
            <div class="idea-footer">
              <div class="idea-meta">
                <span class="meta-timestamp">{{ formatDateTime(idea.created_at) }}</span>
                <span v-if="idea.rating > 0" class="meta-rating">
                  {{ idea.rating }}/5 stars
                </span>
              </div>
              
              <div class="idea-expand-actions">
                <button @click="expandIdea(idea)" class="expand-button">
                  <i class="fas fa-expand-arrows-alt"></i>
                  Expand
                </button>
                <button @click="buildOnIdea(idea)" class="build-button">
                  <i class="fas fa-plus-circle"></i>
                  Build On
                </button>
              </div>
            </div>
          </div>
        </div>
        
        <!-- Empty State -->
        <div v-if="filteredAndSortedIdeas.length === 0" class="empty-state">
          <div class="empty-icon">
            <i class="fas fa-lightbulb"></i>
          </div>
          <h4>No Ideas Yet</h4>
          <p>{{ getEmptyStateMessage() }}</p>
          <button v-if="filterBy !== 'all'" @click="filterBy = 'all'" class="show-all-button">
            Show All Ideas
          </button>
        </div>
      </div>
    </div>

    <!-- Idea Detail Modal -->
    <IdeaDetailModal 
      v-if="selectedIdea"
      :idea="selectedIdea"
      :session="currentSession"
      @close="selectedIdea = null"
      @update="updateIdea"
      @delete="deleteIdea"
      @build-on="buildOnIdea"
    />

    <!-- Generation Progress -->
    <div v-if="isGenerating" class="generation-progress">
      <div class="progress-content">
        <div class="progress-spinner"></div>
        <div class="progress-text">
          <h4>Generating Ideas...</h4>
          <p>AI is brainstorming {{ ideasPerGeneration }} new ideas for you.</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useAdvancedAIStore } from '../../stores/advancedAIStore';
import { useProjectStore } from '../../stores/projectStore';
import IdeaDetailModal from './IdeaDetailModal.vue';
import type { 
  BrainstormSession, 
  BrainstormRequest, 
  BrainstormIdea 
} from '../../types/advancedAI';

// Stores
const advancedAIStore = useAdvancedAIStore();
const projectStore = useProjectStore();

// Reactive state
const selectedSessionId = ref('');
const newSessionTitle = ref('');
const newSessionCategory = ref('general');
const newSessionContext = ref('');
const showSettings = ref(false);
const creativityLevel = ref(7);
const ideasPerGeneration = ref(5);
const useStoryContext = ref(true);
const buildOnExisting = ref(false);
const filterBy = ref('all');
const sortBy = ref('newest');
const viewMode = ref<'cards' | 'list'>('cards');
const editingNotes = ref<string | null>(null);
const noteText = ref('');
const selectedIdea = ref<BrainstormIdea | null>(null);

// Computed properties
const sessions = computed(() => advancedAIStore.brainstormSessions);
const currentSession = computed(() => advancedAIStore.currentBrainstormSession);
const isGenerating = computed(() => advancedAIStore.isGeneratingIdeas);

const canStartSession = computed(() => {
  return newSessionTitle.value.trim().length > 0 && 
         newSessionContext.value.trim().length > 0 &&
         projectStore.currentProject;
});

const keeperCount = computed(() => {
  if (!currentSession.value) return 0;
  return currentSession.value.ideas.filter(idea => idea.is_keeper).length;
});

const filteredAndSortedIdeas = computed(() => {
  if (!currentSession.value) return [];
  
  let ideas = [...currentSession.value.ideas];
  
  // Filter
  switch (filterBy.value) {
    case 'keepers':
      ideas = ideas.filter(idea => idea.is_keeper);
      break;
    case 'rated':
      ideas = ideas.filter(idea => idea.rating > 0);
      break;
    case 'unrated':
      ideas = ideas.filter(idea => idea.rating === 0);
      break;
  }
  
  // Sort
  switch (sortBy.value) {
    case 'newest':
      ideas.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime());
      break;
    case 'oldest':
      ideas.sort((a, b) => new Date(a.created_at).getTime() - new Date(b.created_at).getTime());
      break;
    case 'rating':
      ideas.sort((a, b) => b.rating - a.rating);
      break;
    case 'alphabetical':
      ideas.sort((a, b) => a.title.localeCompare(b.title));
      break;
  }
  
  return ideas;
});

// Methods
const createNewSession = () => {
  selectedSessionId.value = '';
  newSessionTitle.value = '';
  newSessionCategory.value = 'general';
  newSessionContext.value = '';
};

const loadSession = async () => {
  if (selectedSessionId.value) {
    await advancedAIStore.loadBrainstormSession(selectedSessionId.value);
  }
};

const startSession = async () => {
  if (!canStartSession.value || !projectStore.currentProject) return;
  
  try {
    const request: BrainstormRequest = {
      project_id: projectStore.currentProject.id,
      title: newSessionTitle.value,
      category: newSessionCategory.value,
      context: newSessionContext.value,
      creativity_level: creativityLevel.value,
      use_story_context: useStoryContext.value
    };
    
    const session = await advancedAIStore.createBrainstormSession(request);
    selectedSessionId.value = session.id;
    
    // Generate initial ideas
    await generateIdeas();
  } catch (error) {
    console.error('Failed to start brainstorm session:', error);
  }
};

const generateIdeas = async () => {
  if (!currentSession.value) return;
  
  try {
    await advancedAIStore.generateBrainstormIdeas({
      session_id: currentSession.value.id,
      count: parseInt(ideasPerGeneration.value.toString()),
      creativity_level: creativityLevel.value,
      build_on_existing: buildOnExisting.value
    });
  } catch (error) {
    console.error('Failed to generate ideas:', error);
  }
};

const toggleKeeper = async (idea: BrainstormIdea) => {
  try {
    await advancedAIStore.toggleIdeaKeeper(idea.id, !idea.is_keeper);
  } catch (error) {
    console.error('Failed to toggle keeper status:', error);
  }
};

const rateIdea = async (idea: BrainstormIdea, rating: number) => {
  try {
    await advancedAIStore.rateIdea(idea.id, rating);
  } catch (error) {
    console.error('Failed to rate idea:', error);
  }
};

const deleteIdea = async (idea: BrainstormIdea) => {
  if (confirm('Are you sure you want to delete this idea?')) {
    try {
      await advancedAIStore.deleteIdea(idea.id);
    } catch (error) {
      console.error('Failed to delete idea:', error);
    }
  }
};

const startEditingNotes = (idea: BrainstormIdea) => {
  editingNotes.value = idea.id;
  noteText.value = idea.notes || '';
};

const saveNotes = async (idea: BrainstormIdea) => {
  try {
    await advancedAIStore.updateIdeaNotes(idea.id, noteText.value);
    editingNotes.value = null;
    noteText.value = '';
  } catch (error) {
    console.error('Failed to save notes:', error);
  }
};

const cancelNotes = () => {
  editingNotes.value = null;
  noteText.value = '';
};

const expandIdea = (idea: BrainstormIdea) => {
  selectedIdea.value = idea;
};

const buildOnIdea = async (idea: BrainstormIdea) => {
  if (!currentSession.value) return;
  
  try {
    await advancedAIStore.generateBrainstormIdeas({
      session_id: currentSession.value.id,
      count: 3,
      creativity_level: creativityLevel.value,
      build_on_existing: true,
      base_idea_id: idea.id
    });
  } catch (error) {
    console.error('Failed to build on idea:', error);
  }
};

const updateIdea = (updatedIdea: BrainstormIdea) => {
  // Update would be handled by the store
  selectedIdea.value = null;
};

const saveSession = async () => {
  if (!currentSession.value) return;
  
  try {
    await advancedAIStore.saveBrainstormSession(currentSession.value.id);
  } catch (error) {
    console.error('Failed to save session:', error);
  }
};

const exportKeepers = async () => {
  if (!currentSession.value) return;
  
  try {
    await advancedAIStore.exportKeepersToStoryBible(currentSession.value.id);
  } catch (error) {
    console.error('Failed to export keepers:', error);
  }
};

// Utility functions
const formatCategory = (category: string): string => {
  return category.split('_').map(word => 
    word.charAt(0).toUpperCase() + word.slice(1)
  ).join(' ');
};

const formatDate = (dateString: string): string => {
  const date = new Date(dateString);
  return date.toLocaleDateString();
};

const formatDateTime = (dateString: string): string => {
  const date = new Date(dateString);
  return date.toLocaleString();
};

const getEmptyStateMessage = (): string => {
  switch (filterBy.value) {
    case 'keepers':
      return 'No ideas marked as keepers yet. Star your favorite ideas to see them here.';
    case 'rated':
      return 'No rated ideas yet. Rate some ideas to see them here.';
    case 'unrated':
      return 'All ideas have been rated!';
    default:
      return 'Generate some ideas to get started with your brainstorming session.';
  }
};

// Lifecycle
onMounted(async () => {
  if (projectStore.currentProject) {
    await advancedAIStore.loadBrainstormSessions(projectStore.currentProject.id);
  }
});
</script>

<style scoped>
.brainstorm-engine {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  height: 100%;
}

.session-header {
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

.session-controls {
  display: flex;
  gap: 1rem;
  align-items: center;
}

.session-select {
  min-width: 200px;
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
}

.new-session-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 4px;
  background: var(--accent-color);
  color: white;
  cursor: pointer;
  transition: all 0.2s ease;
}

.new-session-button:hover {
  background: var(--accent-color-hover);
}

.session-config {
  background: var(--bg-secondary);
  border-radius: 8px;
  padding: 1.5rem;
  border: 1px solid var(--border-color);
}

.config-form h4 {
  margin: 0 0 1.5rem 0;
  color: var(--text-primary);
}

.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: var(--text-primary);
}

.form-input,
.form-select,
.form-textarea {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-family: inherit;
}

.form-textarea {
  resize: vertical;
  min-height: 80px;
}

.form-actions {
  margin-top: 1.5rem;
}

.start-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 4px;
  background: var(--accent-color);
  color: white;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.start-button:hover:not(:disabled) {
  background: var(--accent-color-hover);
}

.start-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.active-session {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  flex: 1;
}

.session-info {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 1rem;
  padding: 1.5rem;
  background: var(--bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.session-details h4 {
  margin: 0 0 0.5rem 0;
  color: var(--text-primary);
}

.session-meta {
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

.session-actions {
  display: flex;
  gap: 0.5rem;
}

.generate-button,
.export-button,
.save-button {
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

.generate-button {
  background: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.generate-button:hover:not(:disabled) {
  background: var(--accent-color-hover);
}

.generate-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.export-button:hover:not(:disabled),
.save-button:hover {
  background: var(--bg-hover);
}

.export-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.generation-settings {
  background: var(--bg-secondary);
  border-radius: 8px;
  padding: 1rem;
  border: 1px solid var(--border-color);
}

.settings-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
}

.setting-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.setting-group label {
  font-weight: 500;
  color: var(--text-primary);
  font-size: 0.875rem;
}

.slider-container {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.creativity-slider {
  width: 100%;
}

.slider-labels {
  display: flex;
  justify-content: space-between;
  font-size: 0.75rem;
  color: var(--text-secondary);
}

.setting-select {
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
}

.settings-toggle {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
  align-self: flex-start;
}

.settings-toggle:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.ideas-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.ideas-controls {
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

.ideas-container {
  flex: 1;
  overflow-y: auto;
}

.ideas-container.cards {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: 1rem;
}

.ideas-container.list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.idea-item {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 1rem;
  transition: all 0.2s ease;
}

.idea-item:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.idea-item.keeper {
  border-color: #ffd700;
  background: linear-gradient(135deg, var(--bg-secondary) 0%, rgba(255, 215, 0, 0.05) 100%);
}

.idea-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 1rem;
  margin-bottom: 0.75rem;
}

.idea-title {
  font-weight: 600;
  color: var(--text-primary);
  flex: 1;
}

.idea-actions {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.keeper-button {
  padding: 0.25rem;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.keeper-button.active {
  color: #ffd700;
}

.rating-controls {
  display: flex;
  gap: 0.125rem;
}

.rating-star {
  padding: 0.125rem;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 0.75rem;
}

.rating-star.active {
  color: #ffd700;
}

.delete-button {
  padding: 0.25rem;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.delete-button:hover {
  color: #dc3545;
}

.idea-content {
  margin-bottom: 0.75rem;
}

.idea-description {
  margin: 0 0 0.75rem 0;
  color: var(--text-primary);
  line-height: 1.5;
}

.idea-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.25rem;
  margin-bottom: 0.75rem;
}

.idea-tag {
  padding: 0.125rem 0.5rem;
  background: var(--accent-color);
  color: white;
  border-radius: 12px;
  font-size: 0.75rem;
}

.idea-notes {
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

.notes-editor {
  margin-bottom: 0.75rem;
}

.notes-textarea {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-family: inherit;
  resize: vertical;
  margin-bottom: 0.5rem;
}

.notes-actions {
  display: flex;
  gap: 0.5rem;
}

.save-notes-button,
.cancel-notes-button {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.25rem 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: transparent;
  color: var(--text-primary);
  cursor: pointer;
  font-size: 0.75rem;
  transition: all 0.2s ease;
}

.save-notes-button {
  background: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.save-notes-button:hover {
  background: var(--accent-color-hover);
}

.cancel-notes-button:hover {
  background: var(--bg-hover);
}

.add-notes-button {
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

.add-notes-button:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.idea-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1rem;
  padding-top: 0.75rem;
  border-top: 1px solid var(--border-color);
}

.idea-meta {
  display: flex;
  gap: 1rem;
  font-size: 0.75rem;
  color: var(--text-secondary);
}

.idea-expand-actions {
  display: flex;
  gap: 0.5rem;
}

.expand-button,
.build-button {
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

.expand-button:hover,
.build-button:hover {
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

.show-all-button {
  padding: 0.5rem 1rem;
  border: 1px solid var(--accent-color);
  border-radius: 4px;
  background: transparent;
  color: var(--accent-color);
  cursor: pointer;
  transition: all 0.2s ease;
}

.show-all-button:hover {
  background: var(--accent-color);
  color: white;
}

.generation-progress {
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

/* List view specific styles */
.ideas-container.list .idea-item {
  padding: 0.75rem 1rem;
}

.ideas-container.list .idea-header {
  margin-bottom: 0.5rem;
}

.ideas-container.list .idea-content {
  margin-bottom: 0.5rem;
}

.ideas-container.list .idea-footer {
  padding-top: 0.5rem;
}

/* Responsive design */
@media (max-width: 768px) {
  .session-header {
    flex-direction: column;
    align-items: stretch;
  }
  
  .session-controls {
    flex-direction: column;
  }
  
  .session-info {
    flex-direction: column;
  }
  
  .session-actions {
    flex-wrap: wrap;
  }
  
  .settings-grid {
    grid-template-columns: 1fr;
  }
  
  .ideas-controls {
    flex-direction: column;
    align-items: stretch;
  }
  
  .filter-controls {
    justify-content: space-between;
  }
  
  .ideas-container.cards {
    grid-template-columns: 1fr;
  }
  
  .idea-header {
    flex-direction: column;
    align-items: stretch;
    gap: 0.5rem;
  }
  
  .idea-actions {
    justify-content: space-between;
  }
  
  .idea-footer {
    flex-direction: column;
    align-items: stretch;
    gap: 0.5rem;
  }
}
</style>