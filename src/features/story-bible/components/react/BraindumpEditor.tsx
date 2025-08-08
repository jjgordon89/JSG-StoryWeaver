import React, { useState, useEffect } from 'react';
import { Button } from '../../../../components/ui/Button';
import { Card, CardContent, CardHeader, CardTitle } from '../../../../components/ui/Card';
import { Input } from '../../../../components/ui/input';
import { Textarea } from '../../../../components/ui/textarea';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../../../../components/ui/select';
import { Sparkles, Loader2 } from 'lucide-react';
import type { StoryBible, BraindumpEditorProps } from '../../../../types/storyBible';
import useStoryBible from '../../hooks/useStoryBible';

interface BraindumpFormData {
  braindump: string;
  synopsis: string;
  genre: string;
  style: string;
  povMode: string;
  globalTense: string;
  primaryPovCharacter: string;
}

const POV_MODE_OPTIONS = [
  { value: 'first', label: 'First Person' },
  { value: 'second', label: 'Second Person' },
  { value: 'third_limited', label: 'Third Person Limited' },
  { value: 'third_omniscient', label: 'Third Person Omniscient' },
  { value: 'mixed', label: 'Mixed' }
];

const TENSE_OPTIONS = [
  { value: 'past', label: 'Past Tense' },
  { value: 'present', label: 'Present Tense' },
  { value: 'future', label: 'Future Tense' },
  { value: 'mixed', label: 'Mixed' }
];

const GENRE_SUGGESTIONS = [
  'Fantasy',
  'Science Fiction',
  'Mystery',
  'Romance',
  'Thriller',
  'Horror',
  'Historical Fiction',
  'Contemporary Fiction',
  'Young Adult',
  'Literary Fiction',
  'Adventure',
  'Crime',
  'Dystopian',
  'Urban Fantasy',
  'Paranormal Romance'
];

const BraindumpEditor: React.FC<BraindumpEditorProps> = ({ 
  projectId, 
  onSaved 
}) => {
  const { 
    storyBible, 
    isLoading, 
    error, 
    createOrUpdateStoryBible, 
    loadStoryBible,
    generateSynopsis 
  } = useStoryBible();

  const [isEditing, setIsEditing] = useState(false);
  const [isSaving, setIsSaving] = useState(false);
  const [hasChanges, setHasChanges] = useState(false);
  const [isGeneratingSynopsis, setIsGeneratingSynopsis] = useState(false);
  const [formData, setFormData] = useState<BraindumpFormData>({
    braindump: '',
    synopsis: '',
    genre: '',
    style: '',
    povMode: '',
    globalTense: '',
    primaryPovCharacter: ''
  });

  // Load story bible data on mount
  useEffect(() => {
    if (projectId) {
      loadStoryBible(projectId);
    }
  }, [projectId, loadStoryBible]);

  // Update form data when story bible changes
  useEffect(() => {
    if (storyBible) {
      setFormData({
        braindump: storyBible.braindump || '',
        synopsis: storyBible.synopsis || '',
        genre: storyBible.genre || '',
        style: storyBible.style || '',
        povMode: storyBible.pov_mode || '',
        globalTense: storyBible.global_tense || '',
        primaryPovCharacter: storyBible.primary_pov_character || ''
      });
    }
  }, [storyBible]);

  const handleInputChange = (field: keyof BraindumpFormData, value: string) => {
    setFormData(prev => ({ ...prev, [field]: value }));
    setHasChanges(true);
  };

  const startEditing = () => {
    setIsEditing(true);
    setHasChanges(false);
  };

  const cancelEditing = () => {
    setIsEditing(false);
    setHasChanges(false);
    
    // Reset form data to original values
    if (storyBible) {
      setFormData({
        braindump: storyBible.braindump || '',
        synopsis: storyBible.synopsis || '',
        genre: storyBible.genre || '',
        style: storyBible.style || '',
        povMode: storyBible.pov_mode || '',
        globalTense: storyBible.global_tense || '',
        primaryPovCharacter: storyBible.primary_pov_character || ''
      });
    }
  };

  const saveChanges = async () => {
    if (!projectId) return;
    
    setIsSaving(true);
    
    try {
      const request = {
        project_id: projectId,
        braindump: formData.braindump,
        synopsis: formData.synopsis,
        genre: formData.genre,
        style: formData.style,
        pov_mode: formData.povMode,
        global_tense: formData.globalTense,
        primary_pov_character: formData.primaryPovCharacter,
        ...(storyBible?.id && { id: storyBible.id })
      };
      
      await createOrUpdateStoryBible(request);
      
      setIsEditing(false);
      setHasChanges(false);
      
      if (onSaved) {
        onSaved();
      }
    } catch (err) {
      console.error('Failed to save story bible:', err);
    } finally {
      setIsSaving(false);
    }
  };

  const handleGenerateSynopsis = async () => {
    if (!projectId) return;
    
    setIsGeneratingSynopsis(true);
    
    try {
      const request = {
        project_id: projectId,
        braindump: formData.braindump,
        genre: formData.genre,
        style: formData.style
      };
      
      const generatedSynopsis = await generateSynopsis(request);
      
      if (generatedSynopsis) {
        handleInputChange('synopsis', generatedSynopsis);
      }
    } catch (err) {
      console.error('Failed to generate synopsis:', err);
    } finally {
      setIsGeneratingSynopsis(false);
    }
  };

  const getPovModeLabel = (value: string): string => {
    return POV_MODE_OPTIONS.find(option => option.value === value)?.label || value;
  };

  const getTenseLabel = (value: string): string => {
    return TENSE_OPTIONS.find(option => option.value === value)?.label || value;
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center p-8">
        <div className="text-gray-500">Loading story bible...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="p-4 bg-red-50 border border-red-200 rounded-md">
        <div className="text-red-800">Error loading story bible: {error}</div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-bold text-gray-900">Story Bible</h2>
        <div className="flex gap-2">
          {!isEditing ? (
            <Button onClick={startEditing} variant="primary">
              Edit
            </Button>
          ) : (
            <>
              <Button 
                onClick={cancelEditing} 
                variant="outline"
                disabled={isSaving}
              >
                Cancel
              </Button>
              <Button 
                onClick={saveChanges} 
                variant="primary"
                disabled={!hasChanges || isSaving}
              >
                {isSaving ? 'Saving...' : 'Save Changes'}
              </Button>
            </>
          )}
        </div>
      </div>

      {/* Story Metadata */}
      <Card>
        <CardHeader>
          <CardTitle>Story Metadata</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          {/* Genre */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Genre
            </label>
            {isEditing ? (
              <div className="space-y-2">
                <Input
                  value={formData.genre}
                  onChange={(e) => handleInputChange('genre', e.target.value)}
                  placeholder="Enter genre or select from suggestions"
                />
                <div className="flex flex-wrap gap-1">
                  {GENRE_SUGGESTIONS.map(genre => (
                    <Button
                      key={genre}
                      variant="ghost"
                      size="sm"
                      onClick={() => handleInputChange('genre', genre)}
                      className="text-xs"
                    >
                      {genre}
                    </Button>
                  ))}
                </div>
              </div>
            ) : (
              <div className="text-gray-900">
                {formData.genre || 'Not specified'}
              </div>
            )}
          </div>

          {/* POV Mode */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Point of View Mode
            </label>
            {isEditing ? (
              <Select 
                value={formData.povMode} 
                onValueChange={(value) => handleInputChange('povMode', value)}
              >
                <SelectTrigger>
                  <SelectValue placeholder="Select POV mode" />
                </SelectTrigger>
                <SelectContent>
                  {POV_MODE_OPTIONS.map(option => (
                    <SelectItem key={option.value} value={option.value}>
                      {option.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            ) : (
              <div className="text-gray-900">
                {getPovModeLabel(formData.povMode) || 'Not specified'}
              </div>
            )}
          </div>

          {/* Global Tense */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Global Tense
            </label>
            {isEditing ? (
              <Select 
                value={formData.globalTense} 
                onValueChange={(value) => handleInputChange('globalTense', value)}
              >
                <SelectTrigger>
                  <SelectValue placeholder="Select tense" />
                </SelectTrigger>
                <SelectContent>
                  {TENSE_OPTIONS.map(option => (
                    <SelectItem key={option.value} value={option.value}>
                      {option.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            ) : (
              <div className="text-gray-900">
                {getTenseLabel(formData.globalTense) || 'Not specified'}
              </div>
            )}
          </div>

          {/* Primary POV Character */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Primary POV Character
            </label>
            {isEditing ? (
              <Input
                value={formData.primaryPovCharacter}
                onChange={(e) => handleInputChange('primaryPovCharacter', e.target.value)}
                placeholder="Enter primary POV character name"
              />
            ) : (
              <div className="text-gray-900">
                {formData.primaryPovCharacter || 'Not specified'}
              </div>
            )}
          </div>
        </CardContent>
      </Card>

      {/* Synopsis */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle>Synopsis</CardTitle>
            {isEditing && (
              <Button
                onClick={handleGenerateSynopsis}
                disabled={isGeneratingSynopsis || !formData.braindump.trim()}
                variant="outline"
                size="sm"
                className="flex items-center gap-2"
              >
                {isGeneratingSynopsis ? (
                  <Loader2 className="h-4 w-4 animate-spin" />
                ) : (
                  <Sparkles className="h-4 w-4" />
                )}
                {isGeneratingSynopsis ? 'Generating...' : 'Generate with AI'}
              </Button>
            )}
          </div>
        </CardHeader>
        <CardContent>
          {isEditing ? (
            <div className="space-y-2">
              <Textarea
                value={formData.synopsis}
                onChange={(e) => handleInputChange('synopsis', e.target.value)}
                placeholder="Write a brief synopsis of your story..."
                rows={4}
              />
              {!formData.braindump.trim() && (
                <p className="text-sm text-gray-500">
                  💡 Add content to your braindump to enable AI synopsis generation
                </p>
              )}
            </div>
          ) : (
            <div className="text-gray-900 whitespace-pre-wrap">
              {formData.synopsis || 'No synopsis written yet.'}
            </div>
          )}
        </CardContent>
      </Card>

      {/* Writing Style */}
      <Card>
        <CardHeader>
          <CardTitle>Writing Style</CardTitle>
        </CardHeader>
        <CardContent>
          {isEditing ? (
            <Textarea
              value={formData.style}
              onChange={(e) => handleInputChange('style', e.target.value)}
              placeholder="Describe your writing style, tone, and approach..."
              rows={3}
            />
          ) : (
            <div className="text-gray-900 whitespace-pre-wrap">
              {formData.style || 'No writing style notes yet.'}
            </div>
          )}
        </CardContent>
      </Card>

      {/* Braindump */}
      <Card>
        <CardHeader>
          <CardTitle>Braindump</CardTitle>
        </CardHeader>
        <CardContent>
          {isEditing ? (
            <Textarea
              value={formData.braindump}
              onChange={(e) => handleInputChange('braindump', e.target.value)}
              placeholder="Dump all your story ideas, thoughts, and notes here..."
              rows={8}
            />
          ) : (
            <div className="text-gray-900 whitespace-pre-wrap">
              {formData.braindump || 'No braindump content yet. Click Edit to start adding your story ideas!'}
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
};

export default BraindumpEditor;