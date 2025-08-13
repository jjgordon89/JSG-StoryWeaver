import React, { useEffect, useState } from 'react';
import useStoryBible from '../hooks/useStoryBible';
import type {
  CharacterTrait,
  CreateCharacterTraitRequest,
  UpdateCharacterTraitRequest,
  GenerateCharacterTraitsRequest
} from '../../../types/storyBible';
import type { CharacterTemplate } from '../../../lib/types/templates';
import { TemplateService } from '../../../lib/types/templates';
import TemplateSelector from '../../../lib/components/templates/TemplateSelector';
import TemplateApplicationDialog from '../../../lib/components/templates/TemplateApplicationDialog';

// Small local UI primitives (project has larger UI elsewhere; these keep the component self-contained)
const Button: React.FC<{
  variant?: 'primary' | 'outline' | 'ghost' | 'secondary';
  onClick?: () => void;
  disabled?: boolean;
  children?: React.ReactNode;
}> = ({ variant = 'primary', onClick, disabled, children }) => {
  const base = 'inline-flex items-center rounded-md font-medium focus:outline-none';
  const variantCls =
    variant === 'outline'
      ? 'border border-gray-300 bg-white text-gray-700 hover:bg-gray-50'
      : variant === 'ghost'
      ? 'bg-transparent text-gray-700 hover:bg-gray-100'
      : variant === 'secondary'
      ? 'bg-gray-100 text-gray-800 hover:bg-gray-200'
      : 'bg-blue-600 text-white hover:bg-blue-700';
  const disabledCls = disabled ? 'opacity-50 cursor-not-allowed' : '';
  return (
    <button className={`${base} ${variantCls} px-3 py-2 ${disabledCls}`} onClick={onClick} disabled={disabled}>
      {children}
    </button>
  );
};

const Card: React.FC<{ className?: string; children?: React.ReactNode }> = ({ className = '', children }) => (
  <div className={`bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-4 ${className}`}>
    {children}
  </div>
);

const Select: React.FC<{
  id?: string;
  value?: string;
  onChange?: (value: string) => void;
  options?: { value: string; label: string }[];
}> = ({ id, value = '', onChange, options = [] }) => (
  <select
    id={id}
    value={value}
    onChange={(e) => onChange && onChange(e.target.value)}
    className="w-full px-3 py-2 border rounded-md"
  >
    {options.map((o) => (
      <option key={o.value} value={o.value}>
        {o.label}
      </option>
    ))}
  </select>
);

const TextArea: React.FC<{
  id?: string;
  value?: string;
  onChange?: (value: string) => void;
  rows?: number;
}> = ({ id, value = '', onChange, rows = 3 }) => (
  <textarea
    id={id}
    value={value}
    onChange={(e) => onChange && onChange(e.target.value)}
    rows={rows}
    className="w-full px-3 py-2 border rounded-md resize-none"
  />
);

// Local dialog/modal primitive (used in this converted component)
const Dialog: React.FC<{ open: boolean; title?: string; onClose: () => void; children?: React.ReactNode }> = ({ open, title, onClose, children }) => {
  if (!open) return null;
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      <div className="fixed inset-0 bg-black/40" onClick={onClose} />
      <div className="relative w-full max-w-2xl bg-white dark:bg-gray-800 rounded-lg shadow-lg z-10 overflow-auto max-h-[90vh]">
        <div className="p-4 border-b">
          <h3 className="text-lg font-semibold">{title}</h3>
        </div>
        <div className="p-4">{children}</div>
      </div>
    </div>
  );
};

interface Props {
  projectId: string;
  seriesId?: string;
  characterId?: string;
}

const traitTypeOptions = [
  { value: '', label: 'Select trait type' },
  { value: 'physical', label: 'Physical Description' },
  { value: 'personality', label: 'Personality' },
  { value: 'background', label: 'Background' },
  { value: 'motivation', label: 'Motivation' },
  { value: 'goal', label: 'Goals' },
  { value: 'fear', label: 'Fears' },
  { value: 'strength', label: 'Strengths' },
  { value: 'weakness', label: 'Weaknesses' },
  { value: 'relationship', label: 'Relationships' },
  { value: 'quirk', label: 'Quirks' },
  { value: 'secret', label: 'Secrets' },
  { value: 'arc', label: 'Character Arc' },
  { value: 'dialogue', label: 'Dialogue Style' },
  { value: 'other', label: 'Other' }
];

const visibilityOptions = [
  { value: 'always', label: 'Always Visible' },
  { value: 'chapter', label: 'Chapter Context' },
  { value: 'never', label: 'Hidden' }
];

const CharactersManager: React.FC<Props> = ({ projectId, seriesId, characterId }) => {
  const {
    characterTraits,
    characters,
    isLoadingTraits,
    traitsError,
    loadCharacterTraits,
    loadCharacters,
    createCharacterTrait,
    updateCharacterTrait,
    deleteCharacterTrait,
    setSelectedCharacterId,
    setCharacterTraitFilter,
    generateCharacterTraits
  } = useStoryBible();

  // Local UI state
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showEditModal, setShowEditModal] = useState(false);
  const [editingTrait, setEditingTrait] = useState<CharacterTrait | null>(null);
  const [isGeneratingTraits, setIsGeneratingTraits] = useState(false);
  const [showTemplateSelector, setShowTemplateSelector] = useState(false);
  const [showTemplateDialog, setShowTemplateDialog] = useState(false);

  const [characterTemplates, setCharacterTemplates] = useState<any[]>([]);
  const [selectedTemplate, setSelectedTemplate] = useState<any | null>(null);
  const [availableArchetypes, setAvailableArchetypes] = useState<string[]>([]);
  const [isLoadingTemplates, setIsLoadingTemplates] = useState(false);
  const [localFilter, setLocalFilter] = useState<{ traitType?: string; visibility?: string }>({ traitType: '', visibility: '' });

  const [createForm, setCreateForm] = useState<{
    character_id: string;
    trait_name: string;
    trait_value: string;
    visibility: CharacterTrait['visibility'];
  }>({
    character_id: characterId || '',
    trait_name: '',
    trait_value: '',
    visibility: 'always'
  });

  const [editForm, setEditForm] = useState<{
    id: string;
    trait_name: string;
    trait_value: string;
    visibility: CharacterTrait['visibility'];
  }>({
    id: '',
    trait_name: '',
    trait_value: '',
    visibility: 'always'
  });

  // Available characters (from store)
  const availableCharacters = characters.length > 0 ? characters.map((c: any) => ({ id: c.id, name: c.name || 'Character' })) : [
    { id: '1', name: 'Main Character' },
    { id: '2', name: 'Antagonist' },
    { id: '3', name: 'Supporting Character' }
  ];

  useEffect(() => {
    if (characterId) {
      setCreateForm({ ...createForm, character_id: characterId });
      loadCharacterTraits(characterId);
    }
    // load characters for selector
    loadCharacters(projectId);
    // load archetypes
    (async () => {
      try {
        const arch = await TemplateService.getCharacterArchetypes();
        setAvailableArchetypes(arch || []);
      } catch (err) {
        console.error('Failed to load archetypes', err);
      }
    })();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [characterId, projectId]);

  const loadCharacterTemplates = async (archetype: string) => {
    setIsLoadingTemplates(true);
    try {
      const templates = await TemplateService.getCharacterTemplatesByArchetype(archetype);
      setCharacterTemplates(templates || []);
    } catch (err) {
      console.error('Failed to load character templates', err);
    } finally {
      setIsLoadingTemplates(false);
    }
  };

  const openTemplateSelector = () => {
    setShowTemplateSelector(true);
  };

  const handleTemplateSelected = (template: any) => {
    setSelectedTemplate(template as any);
    setShowTemplateSelector(false);
    setShowTemplateDialog(true);
  };

  const handleTemplateApplied = async (payload: { name: string; description?: string; overrides?: Record<string, any> }) => {
    if (!selectedTemplate || !createForm.character_id) return;
    try {
      await TemplateService.applyCharacterTemplate({
        template_id: selectedTemplate.id,
        character_id: createForm.character_id,
        character_name: payload.name,
        character_description: payload.description,
        trait_overrides: payload.overrides
      } as any);
      await loadCharacterTraits(createForm.character_id);
      setShowTemplateDialog(false);
      setSelectedTemplate(null);
    } catch (err) {
      console.error('Failed to apply template', err);
    }
  };

  const openCreateModal = () => {
    setCreateForm({
      character_id: characterId || '',
      trait_name: '',
      trait_value: '',
      visibility: 'always'
    });
    setShowCreateModal(true);
  };

  const openEditModal = (trait: CharacterTrait) => {
    setEditingTrait(trait);
    setEditForm({
      id: trait.id,
      trait_name: trait.trait_name,
      trait_value: trait.trait_value,
      visibility: trait.visibility
    });
    setShowEditModal(true);
  };

  const closeModals = () => {
    setShowCreateModal(false);
    setShowEditModal(false);
    setEditingTrait(null);
  };

  const handleCreateTrait = async () => {
    if (!createForm.character_id || !createForm.trait_name || !createForm.trait_value) return;
    const req: CreateCharacterTraitRequest = {
      character_id: createForm.character_id,
      trait_name: createForm.trait_name,
      trait_value: createForm.trait_value,
      visibility: createForm.visibility
    };
    await createCharacterTrait(req);
    closeModals();
  };

  const handleUpdateTrait = async () => {
    if (!editForm.id || !editForm.trait_name || !editForm.trait_value) return;
    const req: UpdateCharacterTraitRequest = {
      id: editForm.id,
      trait_name: editForm.trait_name,
      trait_value: editForm.trait_value,
      visibility: editForm.visibility
    };
    await updateCharacterTrait(req);
    closeModals();
  };

  const handleDeleteTrait = async (traitId: string) => {
    if (!confirm('Are you sure you want to delete this character trait?')) return;
    await deleteCharacterTrait(traitId);
  };

  const handleCharacterSelect = async (charId: string) => {
    setSelectedCharacterId(charId || null);
    if (charId) await loadCharacterTraits(charId);
  };

  const handleFilterChange = (filterType: string, value: any) => {
    // keep local UI state in sync and propagate to the hook's filter setter
    setLocalFilter((prev) => ({ ...prev, [filterType]: value || '' }));
    // Map filterType to the hook's expected keys
    if (filterType === 'traitType') {
      setCharacterTraitFilter({ traitType: value || undefined });
    } else if (filterType === 'visibility') {
      setCharacterTraitFilter({ visibility: value || undefined });
    }
  };

  const getTraitTypeLabel = (traitType: string) => {
    return traitTypeOptions.find((o) => o.value === traitType)?.label || traitType;
  };

  const getVisibilityLabel = (visibility: CharacterTrait['visibility']) => {
    return visibilityOptions.find((o) => o.value === visibility)?.label || visibility;
  };

  const generateCharacterTraitsHandler = async () => {
    if (!projectId || !createForm.character_id || !createForm.trait_name) return;
    setIsGeneratingTraits(true);
    try {
      const character = availableCharacters.find((c) => c.id === createForm.character_id);
      const characterName = character?.name || 'Character';
      const existingTraits = (characterTraits || []).map((t) => `${t.trait_name}: ${t.trait_value}`);
      const req: GenerateCharacterTraitsRequest = {
        character_id: createForm.character_id,
        character_name: characterName,
        story_context: 'Character trait generation for story bible',
        existing_traits: existingTraits,
        trait_count: 1,
        creativity: 0.7
      };
      const res = await generateCharacterTraits(req);
      if (res?.generated_content) {
        const generated = res.generated_content.split('\n').map(s => s.trim()).filter(Boolean);
        if (generated.length > 0) {
          setCreateForm({ ...createForm, trait_value: generated[0] });
        }
      }
    } catch (err) {
      console.error('Failed to generate character traits', err);
    } finally {
      setIsGeneratingTraits(false);
    }
  };

  return (
    <div className="characters-manager">
      <div className="manager-header flex justify-between items-start p-8 border-b">
        <div>
          <h2 className="text-2xl font-semibold mb-1">Character Traits</h2>
          <p className="text-sm text-gray-600">Manage detailed character information, personality traits, and development notes.</p>
        </div>

        <div className="header-actions flex gap-2">
          <Button variant="outline" onClick={openTemplateSelector} disabled={!createForm.character_id}>
            <span className="mr-2">📋</span>
            Use Template
          </Button>
          <Button variant="primary" onClick={openCreateModal} disabled={!createForm.character_id}>
            <span className="mr-2">➕</span>
            Add Trait
          </Button>
        </div>
      </div>

      <Card className="selection-card my-4">
        <div className="selection-content flex flex-col gap-4">
          <div className="character-selector">
            <label className="block text-sm font-medium mb-1">Select Character:</label>
            <Select
              value={createForm.character_id || ''}
              onChange={(v) => {
                setCreateForm({ ...createForm, character_id: v });
                handleCharacterSelect(v);
              }}
              options={[{ value: '', label: 'Choose a character...' }, ...availableCharacters.map((c) => ({ value: c.id, label: c.name }))]}
            />
          </div>

          {createForm.character_id && (
            <div className="filters flex gap-4 flex-wrap">
              <div className="filter-group min-w-[200px]">
                <label className="block text-sm font-medium mb-1">Filter by Type:</label>
                  <Select
                  value={localFilter.traitType || ''}
                  onChange={(v) => handleFilterChange('traitType', v)}
                  options={[{ value: '', label: 'All types' }, ...traitTypeOptions.slice(1)]}
                />
              </div>

              <div className="filter-group min-w-[200px]">
                <label className="block text-sm font-medium mb-1">Filter by Visibility:</label>
                <Select
                  value={localFilter.visibility || ''}
                  onChange={(v) => handleFilterChange('visibility', v)}
                  options={[{ value: '', label: 'All visibility' }, ...visibilityOptions]}
                />
              </div>
            </div>
          )}
        </div>
      </Card>

      <div className="content-area p-8">
        {traitsError && (
          <div className="mb-4 text-red-600">
            {traitsError}
            <button className="ml-4 underline" onClick={() => { /* clear handled via store */ }}>Dismiss</button>
          </div>
        )}

        {!createForm.character_id ? (
          <div className="empty-state py-12 text-center">
            <div className="text-4xl mb-4">👤</div>
            <h3 className="text-lg font-semibold">Select a Character</h3>
            <p className="text-sm text-gray-600">Choose a character from the dropdown above to view and manage their traits.</p>
          </div>
        ) : isLoadingTraits ? (
          <div className="loading-container py-12 text-center">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto mb-4"></div>
            <p>Loading character traits...</p>
          </div>
        ) : (characterTraits || []).length === 0 ? (
          <div className="empty-state py-12 text-center">
            <div className="text-4xl mb-4">📝</div>
            <h3 className="text-lg font-semibold">No Character Traits</h3>
            <p className="text-sm text-gray-600 mb-4">This character doesn't have any traits yet. Start building their profile!</p>
            <Button variant="primary" onClick={openCreateModal}>Add First Trait</Button>
          </div>
        ) : (
          <div className="traits-grid grid gap-6 grid-cols-1 md:grid-cols-2">
            {(characterTraits || []).map((trait) => (
              <Card key={trait.id} className="trait-card">
                <div className="trait-header flex justify-between items-start mb-2">
                  <div className="trait-meta">
                    <h4 className="trait-type text-lg font-semibold">{getTraitTypeLabel(trait.trait_name)}</h4>
                    <div className="trait-badges flex gap-2 mt-2">
                      <span className={`visibility-badge px-2 py-0.5 rounded text-xs ${trait.visibility === 'always' ? 'bg-green-100 text-green-800' : trait.visibility === 'chapter' ? 'bg-yellow-100 text-yellow-800' : 'bg-red-100 text-red-800'}`}>
                        {getVisibilityLabel(trait.visibility)}
                      </span>
                    </div>
                  </div>

                  <div className="trait-actions flex gap-2">
                    <Button variant="ghost" onClick={() => openEditModal(trait)}>✏️</Button>
                    <Button variant="ghost" onClick={() => handleDeleteTrait(trait.id)}>🗑️</Button>
                  </div>
                </div>

                <div className="trait-content mb-3">
                  <p>{trait.trait_value}</p>
                </div>

                <div className="trait-footer text-sm text-gray-500">
                  Updated {new Date(trait.updated_at).toLocaleDateString()}
                </div>
              </Card>
            ))}
          </div>
        )}
      </div>

      {/* Create Modal */}
      <Dialog open={showCreateModal} title="Add Character Trait" onClose={() => setShowCreateModal(false)}>
        <div className="modal-form flex flex-col gap-4">
          <div>
            <label className="block text-sm font-medium mb-1">Character:</label>
            <Select
              value={createForm.character_id || ''}
              onChange={(v) => setCreateForm({ ...createForm, character_id: v })}
              options={[{ value: '', label: 'Select character...' }, ...availableCharacters.map((c) => ({ value: c.id, label: c.name }))]}
            />
          </div>

          <div>
            <label className="block text-sm font-medium mb-1">Trait Type:</label>
            <Select value={createForm.trait_name} onChange={(v) => setCreateForm({ ...createForm, trait_name: v })} options={traitTypeOptions} />
          </div>

          <div>
            <div className="flex justify-between items-center mb-1">
              <label className="block text-sm font-medium">Content:</label>
              <Button variant="outline" onClick={generateCharacterTraitsHandler} disabled={isGeneratingTraits || !createForm.trait_name}>
                {isGeneratingTraits ? 'Generating...' : '✨ Generate with AI'}
              </Button>
            </div>
            <TextArea value={createForm.trait_value} onChange={(v) => setCreateForm({ ...createForm, trait_value: v })} rows={4} />
            {!createForm.trait_name && <p className="text-sm text-gray-500 italic mt-1">💡 Select a trait type to enable AI generation</p>}
          </div>

          <div>
            <label className="block text-sm font-medium mb-1">Visibility:</label>
            <Select value={createForm.visibility} onChange={(v) => setCreateForm({ ...createForm, visibility: v as any })} options={visibilityOptions} />
          </div>

          <div className="flex justify-end gap-2">
            <Button variant="secondary" onClick={() => setShowCreateModal(false)}>Cancel</Button>
            <Button variant="primary" onClick={handleCreateTrait} disabled={!createForm.character_id || !createForm.trait_name || !createForm.trait_value}>Add Trait</Button>
          </div>
        </div>
      </Dialog>

      {/* Edit Modal */}
      <Dialog open={showEditModal} title="Edit Character Trait" onClose={() => setShowEditModal(false)}>
        <div className="modal-form flex flex-col gap-4">
          <div>
            <label className="block text-sm font-medium mb-1">Trait Type:</label>
            <Select value={editForm.trait_name} onChange={(v) => setEditForm({ ...editForm, trait_name: v })} options={traitTypeOptions} />
          </div>

          <div>
            <label className="block text-sm font-medium mb-1">Content:</label>
            <TextArea value={editForm.trait_value} onChange={(v) => setEditForm({ ...editForm, trait_value: v })} rows={4} />
          </div>

          <div>
            <label className="block text-sm font-medium mb-1">Visibility:</label>
            <Select value={editForm.visibility} onChange={(v) => setEditForm({ ...editForm, visibility: v as any })} options={visibilityOptions} />
          </div>

          <div className="flex justify-end gap-2">
            <Button variant="secondary" onClick={() => setShowEditModal(false)}>Cancel</Button>
            <Button variant="primary" onClick={handleUpdateTrait} disabled={!editForm.trait_name || !editForm.trait_value}>Save Changes</Button>
          </div>
        </div>
      </Dialog>

      {/* Template Selector Modal */}
      <Dialog open={showTemplateSelector} title="Select Character Template" onClose={() => setShowTemplateSelector(false)}>
        <TemplateSelector
          templates={characterTemplates}
          templateType="character"
          selectedTemplateId={null}
          isLoading={isLoadingTemplates}
          onSelect={(id, t) => handleTemplateSelected(t)}
          onApply={() => {}}
        />
        <div className="flex justify-end gap-2 mt-4">
          <Button variant="secondary" onClick={() => setShowTemplateSelector(false)}>Cancel</Button>
        </div>
      </Dialog>

      {/* Template Application Dialog */}
      {selectedTemplate && (
        <TemplateApplicationDialog
          open={showTemplateDialog}
          template={selectedTemplate}
          templateType="character"
          projectId={projectId}
          isApplying={false}
          onApply={(req) => handleTemplateApplied(req as any)}
          onCancel={() => { setShowTemplateDialog(false); setSelectedTemplate(null); }}
        />
      )}
    </div>
  );
};

export default CharactersManager;
