<template>
  <div class="idea-detail-modal-overlay" @click="closeModal">
    <div class="idea-detail-modal" @click.stop>
      <!-- Modal Header -->
      <div class="modal-header">
        <div class="header-content">
          <h3>{{ idea.title }}</h3>
          <div class="idea-meta">
            <span class="meta-item">
              <i class="fas fa-calendar"></i>
              {{ formatDateTime(idea.created_at) }}
            </span>
            <span v-if="idea.rating > 0" class="meta-item">
              <i class="fas fa-star"></i>
              {{ idea.rating }}/5 stars
            </span>
            <span v-if="idea.is_keeper" class="meta-item keeper">
              <i class="fas fa-star"></i>
              Keeper
            </span>
          </div>
        </div>
        
        <button @click="closeModal" class="close-button">
          <i class="fas fa-times"></i>
        </button>
      </div>

      <!-- Modal Content -->
      <div class="modal-content">
        <!-- Idea Description -->
        <div class="content-section">
          <h4>Description</h4>
          <div class="description-content">
            <p v-if="!editingDescription" class="description-text">{{ idea.description }}</p>
            <div v-else class="description-editor">
              <textarea 
                v-model="editedDescription"
                rows="4"
                class="description-textarea"
                placeholder="Enter idea description..."
              ></textarea>
              <div class="editor-actions">
                <button @click="saveDescription" class="save-button">
                  <i class="fas fa-check"></i>
                  Save
                </button>
                <button @click="cancelEditDescription" class="cancel-button">
                  <i class="fas fa-times"></i>
                  Cancel
                </button>
              </div>
            </div>
            <button 
              v-if="!editingDescription"
              @click="startEditDescription" 
              class="edit-button"
            >
              <i class="fas fa-edit"></i>
              Edit
            </button>
          </div>
        </div>

        <!-- Tags -->
        <div class="content-section">
          <h4>Tags</h4>
          <div class="tags-content">
            <div class="tags-display">
              <span 
                v-for="tag in idea.tags" 
                :key="tag"
                class="tag-item"
              >
                {{ tag }}
                <button @click="removeTag(tag)" class="tag-remove">
                  <i class="fas fa-times"></i>
                </button>
              </span>
              <div v-if="idea.tags.length === 0" class="no-tags">
                No tags yet
              </div>
            </div>
            
            <div class="tag-editor">
              <input 
                v-model="newTag"
                @keyup.enter="addTag"
                type="text"
                placeholder="Add a tag..."
                class="tag-input"
              >
              <button @click="addTag" :disabled="!newTag.trim()" class="add-tag-button">
                <i class="fas fa-plus"></i>
                Add
              </button>
            </div>
          </div>
        </div>

        <!-- Notes -->
        <div class="content-section">
          <h4>Notes</h4>
          <div class="notes-content">
            <div v-if="!editingNotes && idea.notes" class="notes-display">
              <p class="notes-text">{{ idea.notes }}</p>
              <button @click="startEditNotes" class="edit-button">
                <i class="fas fa-edit"></i>
                Edit Notes
              </button>
            </div>
            
            <div v-else-if="editingNotes" class="notes-editor">
              <textarea 
                v-model="editedNotes"
                rows="4"
                class="notes-textarea"
                placeholder="Add your notes about this idea..."
              ></textarea>
              <div class="editor-actions">
                <button @click="saveNotes" class="save-button">
                  <i class="fas fa-check"></i>
                  Save
                </button>
                <button @click="cancelEditNotes" class="cancel-button">
                  <i class="fas fa-times"></i>
                  Cancel
                </button>
              </div>
            </div>
            
            <div v-else class="no-notes">
              <p>No notes yet</p>
              <button @click="startEditNotes" class="add-notes-button">
                <i class="fas fa-plus"></i>
                Add Notes
              </button>
            </div>
          </div>
        </div>

        <!-- Rating -->
        <div class="content-section">
          <h4>Rating</h4>
          <div class="rating-content">
            <div class="rating-stars">
              <button 
                v-for="rating in 5" 
                :key="rating"
                @click="updateRating(rating)"
                :class="['rating-star', { active: rating <= idea.rating }]"
              >
                <i class="fas fa-star"></i>
              </button>
            </div>
            <span class="rating-text">
              {{ idea.rating > 0 ? `${idea.rating}/5 stars` : 'Not rated' }}
            </span>
          </div>
        </div>

        <!-- Keeper Status -->
        <div class="content-section">
          <h4>Keeper Status</h4>
          <div class="keeper-content">
            <label class="keeper-toggle">
              <input 
                type="checkbox" 
                :checked="idea.is_keeper"
                @change="toggleKeeper"
              >
              <span class="toggle-slider"></span>
              <span class="toggle-label">
                {{ idea.is_keeper ? 'This is a keeper idea' : 'Mark as keeper' }}
              </span>
            </label>
            <p class="keeper-description">
              Keeper ideas can be exported to your Story Bible for future reference.
            </p>
          </div>
        </div>

        <!-- Usage Suggestions -->
        <div class="content-section" v-if="usageSuggestions.length > 0">
          <h4>Usage Suggestions</h4>
          <div class="suggestions-content">
            <div 
              v-for="suggestion in usageSuggestions" 
              :key="suggestion.type"
              class="suggestion-item"
            >
              <div class="suggestion-header">
                <i :class="suggestion.icon"></i>
                <span class="suggestion-title">{{ suggestion.title }}</span>
              </div>
              <p class="suggestion-description">{{ suggestion.description }}</p>
              <button 
                @click="applySuggestion(suggestion)"
                class="suggestion-button"
              >
                {{ suggestion.action }}
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Modal Footer -->
      <div class="modal-footer">
        <div class="footer-actions">
          <button @click="buildOnIdea" class="action-button build-button">
            <i class="fas fa-plus-circle"></i>
            Build On This Idea
          </button>
          
          <button @click="duplicateIdea" class="action-button duplicate-button">
            <i class="fas fa-copy"></i>
            Duplicate
          </button>
          
          <button @click="exportIdea" class="action-button export-button">
            <i class="fas fa-download"></i>
            Export
          </button>
        </div>
        
        <div class="footer-actions">
          <button @click="deleteIdea" class="action-button delete-button">
            <i class="fas fa-trash"></i>
            Delete
          </button>
          
          <button @click="closeModal" class="action-button close-button">
            Close
          </button>
        </div>
      </div>
    </div>

    <!-- Delete Confirmation -->
    <div v-if="showDeleteConfirm" class="delete-confirmation" @click.stop>
      <div class="confirm-content">
        <h4>Delete Idea</h4>
        <p>Are you sure you want to delete "{{ idea.title }}"? This action cannot be undone.</p>
        <div class="confirm-actions">
          <button @click="confirmDelete" class="confirm-delete-button">
            <i class="fas fa-trash"></i>
            Delete
          </button>
          <button @click="showDeleteConfirm = false" class="cancel-delete-button">
            Cancel
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import type { BrainstormIdea, BrainstormSession } from '../../types/advancedAI';

// Props
interface Props {
  idea: BrainstormIdea;
  session: BrainstormSession;
}

const props = defineProps<Props>();

// Emits
interface Emits {
  close: [];
  update: [idea: BrainstormIdea];
  delete: [idea: BrainstormIdea];
  buildOn: [idea: BrainstormIdea];
}

const emit = defineEmits<Emits>();

// Reactive state
const editingDescription = ref(false);
const editedDescription = ref('');
const editingNotes = ref(false);
const editedNotes = ref('');
const newTag = ref('');
const showDeleteConfirm = ref(false);

// Computed properties
const usageSuggestions = computed(() => {
  const suggestions = [];
  
  // Category-based suggestions
  switch (props.session.category) {
    case 'characters':
      suggestions.push({
        type: 'character',
        icon: 'fas fa-user',
        title: 'Add to Character Bible',
        description: 'Create a new character entry in your Story Bible based on this idea.',
        action: 'Create Character'
      });
      break;
    case 'plot':
      suggestions.push({
        type: 'plot',
        icon: 'fas fa-book',
        title: 'Add to Plot Outline',
        description: 'Incorporate this idea into your story\'s plot structure.',
        action: 'Add to Plot'
      });
      break;
    case 'worldbuilding':
      suggestions.push({
        type: 'world',
        icon: 'fas fa-globe',
        title: 'Add to World Bible',
        description: 'Create a world-building entry based on this idea.',
        action: 'Add to World'
      });
      break;
    case 'scenes':
      suggestions.push({
        type: 'scene',
        icon: 'fas fa-film',
        title: 'Create Scene Outline',
        description: 'Turn this idea into a detailed scene outline.',
        action: 'Create Scene'
      });
      break;
  }
  
  // Rating-based suggestions
  if (props.idea.rating >= 4) {
    suggestions.push({
      type: 'develop',
      icon: 'fas fa-expand-arrows-alt',
      title: 'Develop Further',
      description: 'This highly-rated idea deserves more development.',
      action: 'Expand Idea'
    });
  }
  
  // Tag-based suggestions
  if (props.idea.tags.includes('conflict') || props.idea.tags.includes('tension')) {
    suggestions.push({
      type: 'conflict',
      icon: 'fas fa-bolt',
      title: 'Conflict Development',
      description: 'Use this idea to create dramatic tension in your story.',
      action: 'Develop Conflict'
    });
  }
  
  return suggestions;
});

// Methods
const closeModal = () => {
  emit('close');
};

const startEditDescription = () => {
  editingDescription.value = true;
  editedDescription.value = props.idea.description;
};

const saveDescription = () => {
  if (editedDescription.value.trim()) {
    const updatedIdea = {
      ...props.idea,
      description: editedDescription.value.trim()
    };
    emit('update', updatedIdea);
    editingDescription.value = false;
  }
};

const cancelEditDescription = () => {
  editingDescription.value = false;
  editedDescription.value = '';
};

const startEditNotes = () => {
  editingNotes.value = true;
  editedNotes.value = props.idea.notes || '';
};

const saveNotes = () => {
  const updatedIdea = {
    ...props.idea,
    notes: editedNotes.value.trim() || null
  };
  emit('update', updatedIdea);
  editingNotes.value = false;
};

const cancelEditNotes = () => {
  editingNotes.value = false;
  editedNotes.value = '';
};

const addTag = () => {
  const tag = newTag.value.trim().toLowerCase();
  if (tag && !props.idea.tags.includes(tag)) {
    const updatedIdea = {
      ...props.idea,
      tags: [...props.idea.tags, tag]
    };
    emit('update', updatedIdea);
    newTag.value = '';
  }
};

const removeTag = (tagToRemove: string) => {
  const updatedIdea = {
    ...props.idea,
    tags: props.idea.tags.filter(tag => tag !== tagToRemove)
  };
  emit('update', updatedIdea);
};

const updateRating = (rating: number) => {
  const newRating = props.idea.rating === rating ? 0 : rating;
  const updatedIdea = {
    ...props.idea,
    rating: newRating
  };
  emit('update', updatedIdea);
};

const toggleKeeper = () => {
  const updatedIdea = {
    ...props.idea,
    is_keeper: !props.idea.is_keeper
  };
  emit('update', updatedIdea);
};

const buildOnIdea = () => {
  emit('buildOn', props.idea);
  closeModal();
};

const duplicateIdea = () => {
  const duplicatedIdea = {
    ...props.idea,
    id: `${props.idea.id}_copy_${Date.now()}`,
    title: `${props.idea.title} (Copy)`,
    created_at: new Date().toISOString(),
    is_keeper: false,
    rating: 0
  };
  emit('update', duplicatedIdea);
};

const exportIdea = () => {
  const ideaText = `# ${props.idea.title}\n\n${props.idea.description}\n\n**Tags:** ${props.idea.tags.join(', ')}\n\n**Rating:** ${props.idea.rating}/5\n\n**Notes:** ${props.idea.notes || 'None'}`;
  
  const blob = new Blob([ideaText], { type: 'text/markdown' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = `${props.idea.title.replace(/[^a-z0-9]/gi, '_').toLowerCase()}.md`;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
};

const deleteIdea = () => {
  showDeleteConfirm.value = true;
};

const confirmDelete = () => {
  emit('delete', props.idea);
  closeModal();
};

const applySuggestion = (suggestion: any) => {
  // This would integrate with the Story Bible or other components
  console.log('Applying suggestion:', suggestion.type, 'for idea:', props.idea.title);
  // Implementation would depend on the specific suggestion type
};

// Utility functions
const formatDateTime = (dateString: string): string => {
  const date = new Date(dateString);
  return date.toLocaleString();
};

// Lifecycle
onMounted(() => {
  // Focus management or other setup
});
</script>

<style scoped>
.idea-detail-modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 1rem;
}

.idea-detail-modal {
  background: var(--bg-primary);
  border-radius: 12px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  max-width: 800px;
  width: 100%;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 1rem;
  padding: 1.5rem;
  border-bottom: 1px solid var(--border-color);
  background: var(--bg-secondary);
}

.header-content h3 {
  margin: 0 0 0.5rem 0;
  color: var(--text-primary);
  font-size: 1.25rem;
}

.idea-meta {
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

.meta-item.keeper {
  color: #ffd700;
  font-weight: 500;
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

.modal-content {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
}

.content-section {
  margin-bottom: 2rem;
}

.content-section:last-child {
  margin-bottom: 0;
}

.content-section h4 {
  margin: 0 0 1rem 0;
  color: var(--text-primary);
  font-size: 1rem;
  font-weight: 600;
}

.description-content,
.notes-content,
.tags-content,
.rating-content,
.keeper-content {
  position: relative;
}

.description-text,
.notes-text {
  margin: 0 0 1rem 0;
  color: var(--text-primary);
  line-height: 1.6;
  padding: 1rem;
  background: var(--bg-secondary);
  border-radius: 6px;
  border: 1px solid var(--border-color);
}

.description-editor,
.notes-editor {
  margin-bottom: 1rem;
}

.description-textarea,
.notes-textarea {
  width: 100%;
  padding: 1rem;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  font-family: inherit;
  resize: vertical;
  margin-bottom: 0.5rem;
}

.editor-actions {
  display: flex;
  gap: 0.5rem;
}

.save-button,
.cancel-button {
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

.save-button {
  background: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.save-button:hover {
  background: var(--accent-color-hover);
}

.cancel-button:hover {
  background: var(--bg-hover);
}

.edit-button,
.add-notes-button {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.5rem 1rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 0.875rem;
  transition: all 0.2s ease;
}

.edit-button:hover,
.add-notes-button:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.tags-display {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  margin-bottom: 1rem;
  min-height: 2rem;
  align-items: center;
}

.tag-item {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.25rem 0.75rem;
  background: var(--accent-color);
  color: white;
  border-radius: 16px;
  font-size: 0.875rem;
}

.tag-remove {
  padding: 0.125rem;
  border: none;
  background: transparent;
  color: rgba(255, 255, 255, 0.7);
  cursor: pointer;
  border-radius: 50%;
  transition: all 0.2s ease;
}

.tag-remove:hover {
  background: rgba(255, 255, 255, 0.2);
  color: white;
}

.no-tags {
  color: var(--text-secondary);
  font-style: italic;
}

.tag-editor {
  display: flex;
  gap: 0.5rem;
}

.tag-input {
  flex: 1;
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.add-tag-button {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.5rem 1rem;
  border: 1px solid var(--accent-color);
  border-radius: 4px;
  background: var(--accent-color);
  color: white;
  cursor: pointer;
  font-size: 0.875rem;
  transition: all 0.2s ease;
}

.add-tag-button:hover:not(:disabled) {
  background: var(--accent-color-hover);
}

.add-tag-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.rating-content {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.rating-stars {
  display: flex;
  gap: 0.25rem;
}

.rating-star {
  padding: 0.25rem;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 1.25rem;
}

.rating-star.active {
  color: #ffd700;
}

.rating-star:hover {
  transform: scale(1.1);
}

.rating-text {
  font-size: 0.875rem;
  color: var(--text-secondary);
}

.keeper-content {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.keeper-toggle {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  cursor: pointer;
}

.keeper-toggle input[type="checkbox"] {
  display: none;
}

.toggle-slider {
  position: relative;
  width: 50px;
  height: 24px;
  background: var(--border-color);
  border-radius: 12px;
  transition: all 0.3s ease;
}

.toggle-slider::before {
  content: '';
  position: absolute;
  top: 2px;
  left: 2px;
  width: 20px;
  height: 20px;
  background: white;
  border-radius: 50%;
  transition: all 0.3s ease;
}

.keeper-toggle input:checked + .toggle-slider {
  background: #ffd700;
}

.keeper-toggle input:checked + .toggle-slider::before {
  transform: translateX(26px);
}

.toggle-label {
  font-weight: 500;
  color: var(--text-primary);
}

.keeper-description {
  margin: 0;
  font-size: 0.875rem;
  color: var(--text-secondary);
}

.suggestions-content {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.suggestion-item {
  padding: 1rem;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
}

.suggestion-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
}

.suggestion-title {
  font-weight: 500;
  color: var(--text-primary);
}

.suggestion-description {
  margin: 0 0 1rem 0;
  color: var(--text-secondary);
  font-size: 0.875rem;
}

.suggestion-button {
  padding: 0.5rem 1rem;
  border: 1px solid var(--accent-color);
  border-radius: 4px;
  background: transparent;
  color: var(--accent-color);
  cursor: pointer;
  font-size: 0.875rem;
  transition: all 0.2s ease;
}

.suggestion-button:hover {
  background: var(--accent-color);
  color: white;
}

.no-notes {
  text-align: center;
  padding: 2rem;
  color: var(--text-secondary);
}

.no-notes p {
  margin: 0 0 1rem 0;
}

.modal-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1rem;
  padding: 1.5rem;
  border-top: 1px solid var(--border-color);
  background: var(--bg-secondary);
}

.footer-actions {
  display: flex;
  gap: 0.5rem;
}

.action-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: transparent;
  color: var(--text-primary);
  cursor: pointer;
  font-size: 0.875rem;
  transition: all 0.2s ease;
}

.action-button:hover {
  background: var(--bg-hover);
}

.build-button {
  background: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.build-button:hover {
  background: var(--accent-color-hover);
}

.delete-button {
  color: #dc3545;
  border-color: #dc3545;
}

.delete-button:hover {
  background: #dc3545;
  color: white;
}

.delete-confirmation {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 1.5rem;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
  z-index: 1001;
}

.confirm-content h4 {
  margin: 0 0 1rem 0;
  color: var(--text-primary);
}

.confirm-content p {
  margin: 0 0 1.5rem 0;
  color: var(--text-secondary);
}

.confirm-actions {
  display: flex;
  gap: 0.5rem;
  justify-content: flex-end;
}

.confirm-delete-button {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 4px;
  background: #dc3545;
  color: white;
  cursor: pointer;
  font-size: 0.875rem;
  transition: all 0.2s ease;
}

.confirm-delete-button:hover {
  background: #c82333;
}

.cancel-delete-button {
  padding: 0.5rem 1rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: transparent;
  color: var(--text-primary);
  cursor: pointer;
  font-size: 0.875rem;
  transition: all 0.2s ease;
}

.cancel-delete-button:hover {
  background: var(--bg-hover);
}

/* Responsive design */
@media (max-width: 768px) {
  .idea-detail-modal {
    margin: 0.5rem;
    max-height: 95vh;
  }
  
  .modal-header {
    flex-direction: column;
    align-items: stretch;
    gap: 1rem;
  }
  
  .idea-meta {
    justify-content: space-between;
  }
  
  .modal-footer {
    flex-direction: column;
    align-items: stretch;
    gap: 1rem;
  }
  
  .footer-actions {
    justify-content: center;
    flex-wrap: wrap;
  }
  
  .tag-editor {
    flex-direction: column;
  }
  
  .rating-content {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }
}
</style>