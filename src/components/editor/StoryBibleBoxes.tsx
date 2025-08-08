import React, { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '../ui/card';
import { Button } from '../ui/button';
import { Textarea } from '../ui/textarea';
import { Input } from '../ui/input';
import { useStoryBible } from '../../features/story-bible/hooks/useStoryBible';
import type { StoryBible } from '../../types/storyBible';

interface StoryBibleBoxesProps {
  projectId: string;
  isVisible: boolean;
}

interface StoryBibleField {
  id: string;
  title: string;
  icon: string;
  type: 'text' | 'textarea';
  placeholder: string;
  value: string;
  canGenerate?: boolean;
  helpText?: string;
}

export const StoryBibleBoxes: React.FC<StoryBibleBoxesProps> = ({ projectId, isVisible }) => {
  const { storyBible, isLoading, loadStoryBible, createOrUpdateStoryBible, generateSynopsis } = useStoryBible();
  const [isGenerating, setIsGenerating] = useState<Record<string, boolean>>({});
  const [editingField, setEditingField] = useState<string | null>(null);
  const [fieldValues, setFieldValues] = useState<Record<string, string>>({});

  // Load Story Bible data when component mounts or projectId changes
  useEffect(() => {
    if (projectId && isVisible) {
      loadStoryBible(projectId);
    }
  }, [projectId, isVisible, loadStoryBible]);

  // Update field values when story bible data changes
  useEffect(() => {
    if (storyBible) {
      setFieldValues({
        braindump: storyBible.braindump || '',
        synopsis: storyBible.synopsis || '',
        genre: storyBible.genre || '',
        style: storyBible.style || '',
        styleExamples: storyBible.style_examples || '',
        povMode: storyBible.pov_mode || '',
        globalTense: storyBible.global_tense || '',
        globalPov: storyBible.global_pov || ''
      });
    }
  }, [storyBible]);

  const storyBibleFields: StoryBibleField[] = [
    {
      id: 'braindump',
      title: 'Braindump',
      icon: '🧠',
      type: 'textarea',
      placeholder: 'Write down your initial story ideas, themes, and concepts...',
      value: fieldValues.braindump || '',
      helpText: 'Your raw story ideas and concepts. This feeds into all other Story Bible sections.'
    },
    {
      id: 'synopsis',
      title: 'Synopsis',
      icon: '📝',
      type: 'textarea',
      placeholder: 'A brief summary of your story...',
      value: fieldValues.synopsis || '',
      canGenerate: true,
      helpText: 'A concise summary of your story that captures the main plot and themes.'
    },
    {
      id: 'genre',
      title: 'Genre',
      icon: '🎭',
      type: 'text',
      placeholder: 'e.g., Fantasy, Science Fiction, Romance...',
      value: fieldValues.genre || '',
      helpText: 'The primary genre of your story, which influences AI suggestions.'
    },
    {
      id: 'style',
      title: 'Writing Style',
      icon: '✍️',
      type: 'textarea',
      placeholder: 'Describe your desired writing style and voice...',
      value: fieldValues.style || '',
      helpText: 'Describe the tone, voice, and style you want for your writing.'
    },
    {
      id: 'styleExamples',
      title: 'Style Examples',
      icon: '📚',
      type: 'textarea',
      placeholder: 'Paste examples of your desired writing style...',
      value: fieldValues.styleExamples || '',
      helpText: 'Examples of writing that match your desired style and voice.'
    },
    {
      id: 'povMode',
      title: 'Point of View',
      icon: '👁️',
      type: 'text',
      placeholder: 'e.g., First Person, Third Person Limited...',
      value: fieldValues.povMode || '',
      helpText: 'The narrative perspective for your story.'
    },
    {
      id: 'globalTense',
      title: 'Narrative Tense',
      icon: '⏰',
      type: 'text',
      placeholder: 'e.g., Past Tense, Present Tense...',
      value: fieldValues.globalTense || '',
      helpText: 'The primary tense used throughout your story.'
    },
    {
      id: 'globalPov',
      title: 'Primary POV Character',
      icon: '👤',
      type: 'text',
      placeholder: 'Main character name...',
      value: fieldValues.globalPov || '',
      helpText: 'The primary point-of-view character for your story.'
    }
  ];

  const handleFieldChange = (fieldId: string, value: string) => {
    setFieldValues(prev => ({ ...prev, [fieldId]: value }));
  };

  const handleSaveField = async (fieldId: string) => {
    try {
      const updatedData: Partial<StoryBible> = {
        project_id: projectId,
        [fieldId === 'styleExamples' ? 'style_examples' : fieldId]: fieldValues[fieldId]
      };

      await createOrUpdateStoryBible(updatedData);
      setEditingField(null);
    } catch (error) {
      console.error('Failed to save field:', error);
    }
  };

  const handleGenerateField = async (fieldId: string) => {
    if (fieldId !== 'synopsis') return;

    setIsGenerating(prev => ({ ...prev, [fieldId]: true }));
    try {
      const result = await generateSynopsis({
        project_id: projectId,
        braindump: fieldValues.braindump,
        genre: fieldValues.genre,
        style: fieldValues.style
      });

      if (result && result.content) {
        setFieldValues(prev => ({ ...prev, synopsis: result.content }));
      }
    } catch (error) {
      console.error('Failed to generate synopsis:', error);
    } finally {
      setIsGenerating(prev => ({ ...prev, [fieldId]: false }));
    }
  };

  if (!isVisible) {
    return null;
  }

  if (isLoading) {
    return (
      <div className="border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-900 p-6">
        <div className="flex items-center justify-center">
          <div className="animate-spin rounded-full h-6 w-6 border-b-2 border-blue-600"></div>
          <span className="ml-2 text-gray-600 dark:text-gray-400">Loading Story Bible...</span>
        </div>
      </div>
    );
  }

  return (
    <div className="border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-900">
      <div className="p-6">
        <div className="flex items-center justify-between mb-6">
          <div>
            <h3 className="text-lg font-semibold text-gray-900 dark:text-gray-100 flex items-center gap-2">
              <span>📚</span>
              Story Bible
            </h3>
            <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
              Core story elements that persist across all documents in this project
            </p>
          </div>
          <Button
            variant="outline"
            size="sm"
            onClick={() => {
              // Navigate to full Story Bible view
              window.location.hash = '#/story-bible';
            }}
          >
            Open Full Editor
          </Button>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {storyBibleFields.map((field) => (
            <Card key={field.id} className="h-fit">
              <CardHeader className="pb-3">
                <CardTitle className="text-sm font-medium flex items-center justify-between">
                  <span className="flex items-center gap-2">
                    <span>{field.icon}</span>
                    {field.title}
                  </span>
                  <div className="flex items-center gap-1">
                    {field.canGenerate && (
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => handleGenerateField(field.id)}
                        disabled={isGenerating[field.id]}
                        className="h-6 w-6 p-0"
                        title="Generate with AI"
                      >
                        {isGenerating[field.id] ? (
                          <div className="animate-spin rounded-full h-3 w-3 border border-gray-400 border-t-transparent"></div>
                        ) : (
                          <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                            <path d="M12 2L2 7l10 5 10-5-10-5z"></path>
                            <path d="M2 17l10 5 10-5"></path>
                            <path d="M2 12l10 5 10-5"></path>
                          </svg>
                        )}
                      </Button>
                    )}
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => setEditingField(editingField === field.id ? null : field.id)}
                      className="h-6 w-6 p-0"
                      title={editingField === field.id ? "Cancel" : "Edit"}
                    >
                      {editingField === field.id ? (
                        <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                          <line x1="18" y1="6" x2="6" y2="18"></line>
                          <line x1="6" y1="6" x2="18" y2="18"></line>
                        </svg>
                      ) : (
                        <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                          <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path>
                          <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path>
                        </svg>
                      )}
                    </Button>
                  </div>
                </CardTitle>
                {field.helpText && (
                  <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
                    {field.helpText}
                  </p>
                )}
              </CardHeader>
              <CardContent className="pt-0">
                {editingField === field.id ? (
                  <div className="space-y-2">
                    {field.type === 'textarea' ? (
                      <Textarea
                        value={fieldValues[field.id] || ''}
                        onChange={(e) => handleFieldChange(field.id, e.target.value)}
                        placeholder={field.placeholder}
                        rows={4}
                        className="text-sm"
                      />
                    ) : (
                      <Input
                        value={fieldValues[field.id] || ''}
                        onChange={(e) => handleFieldChange(field.id, e.target.value)}
                        placeholder={field.placeholder}
                        className="text-sm"
                      />
                    )}
                    <div className="flex gap-2">
                      <Button
                        size="sm"
                        onClick={() => handleSaveField(field.id)}
                        className="flex-1"
                      >
                        Save
                      </Button>
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => {
                          setEditingField(null);
                          // Reset to original value
                          if (storyBible) {
                            const originalValue = field.id === 'styleExamples' 
                              ? storyBible.style_examples 
                              : storyBible[field.id as keyof StoryBible];
                            setFieldValues(prev => ({ 
                              ...prev, 
                              [field.id]: originalValue || '' 
                            }));
                          }
                        }}
                        className="flex-1"
                      >
                        Cancel
                      </Button>
                    </div>
                  </div>
                ) : (
                  <div className="min-h-[60px] flex items-start">
                    {field.value ? (
                      <p className="text-sm text-gray-700 dark:text-gray-300 whitespace-pre-wrap break-words">
                        {field.value.length > 150 ? `${field.value.substring(0, 150)}...` : field.value}
                      </p>
                    ) : (
                      <p className="text-sm text-gray-400 dark:text-gray-500 italic">
                        {field.placeholder}
                      </p>
                    )}
                  </div>
                )}
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    </div>
  );
};

export default StoryBibleBoxes;