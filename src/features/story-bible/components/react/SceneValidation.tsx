import React, { useState, useEffect } from 'react';
import { Button } from '../../../../components/ui/Button';
import { Card, CardContent, CardHeader, CardTitle } from '../../../../components/ui/Card';
import { Badge } from '../../../../components/ui/badge';
import { Progress } from '../../../../components/ui/progress';
import { AlertTriangle, CheckCircle, Clock, Target, Users, MapPin, Eye, RefreshCw } from 'lucide-react';

interface Scene {
  id: string;
  title: string;
  content: string;
  target_word_count?: number;
  pov_character?: string;
  location?: string;
  characters?: string[];
  timeline_position?: number;
  chapter_id?: string;
}

interface ValidationIssue {
  type: 'error' | 'warning' | 'info';
  category: 'word_count' | 'consistency' | 'timeline' | 'characters' | 'pov' | 'structure';
  message: string;
  suggestion?: string;
  scene_id: string;
  scene_title: string;
}

interface SceneValidationProps {
  scenes: Scene[];
  characters: Array<{ id: string; name: string }>;
  worldElements: Array<{ id: string; name: string; element_type: string }>;
  onValidationComplete?: (issues: ValidationIssue[]) => void;
}

const SceneValidation: React.FC<SceneValidationProps> = ({
  scenes,
  characters,
  worldElements,
  onValidationComplete
}) => {
  const [validationIssues, setValidationIssues] = useState<ValidationIssue[]>([]);
  const [isValidating, setIsValidating] = useState(false);
  const [selectedCategory, setSelectedCategory] = useState<string>('all');
  const [validationStats, setValidationStats] = useState({
    total_scenes: 0,
    total_words: 0,
    avg_words_per_scene: 0,
    scenes_with_targets: 0,
    scenes_meeting_targets: 0
  });

  const categories = [
    { id: 'all', label: 'All Issues', icon: Eye },
    { id: 'word_count', label: 'Word Count', icon: Target },
    { id: 'consistency', label: 'Consistency', icon: CheckCircle },
    { id: 'timeline', label: 'Timeline', icon: Clock },
    { id: 'characters', label: 'Characters', icon: Users },
    { id: 'pov', label: 'POV', icon: Eye },
    { id: 'structure', label: 'Structure', icon: MapPin }
  ];

  const validateScenes = async () => {
    setIsValidating(true);
    const issues: ValidationIssue[] = [];
    
    let totalWords = 0;
    let scenesWithTargets = 0;
    let scenesMeetingTargets = 0;

    for (const scene of scenes) {
      const wordCount = countWords(scene.content);
      totalWords += wordCount;

      // Word count validation
      if (scene.target_word_count) {
        scenesWithTargets++;
        const targetDifference = Math.abs(wordCount - scene.target_word_count);
        const percentageDifference = (targetDifference / scene.target_word_count) * 100;

        if (percentageDifference > 50) {
          issues.push({
            type: 'error',
            category: 'word_count',
            message: `Scene is ${percentageDifference.toFixed(1)}% ${wordCount > scene.target_word_count ? 'over' : 'under'} target word count`,
            suggestion: `Target: ${scene.target_word_count} words, Current: ${wordCount} words`,
            scene_id: scene.id,
            scene_title: scene.title
          });
        } else if (percentageDifference > 25) {
          issues.push({
            type: 'warning',
            category: 'word_count',
            message: `Scene is ${percentageDifference.toFixed(1)}% ${wordCount > scene.target_word_count ? 'over' : 'under'} target word count`,
            suggestion: `Consider adjusting content to meet target of ${scene.target_word_count} words`,
            scene_id: scene.id,
            scene_title: scene.title
          });
        } else {
          scenesMeetingTargets++;
        }
      } else {
        issues.push({
          type: 'info',
          category: 'word_count',
          message: 'No target word count set',
          suggestion: 'Consider setting a target word count for better pacing',
          scene_id: scene.id,
          scene_title: scene.title
        });
      }

      // POV consistency validation
      if (scene.pov_character) {
        const povCharacter = characters.find(c => c.id === scene.pov_character || c.name === scene.pov_character);
        if (!povCharacter) {
          issues.push({
            type: 'error',
            category: 'pov',
            message: 'POV character not found in character database',
            suggestion: 'Verify the POV character exists or update the scene',
            scene_id: scene.id,
            scene_title: scene.title
          });
        }
      } else {
        issues.push({
          type: 'warning',
          category: 'pov',
          message: 'No POV character specified',
          suggestion: 'Consider specifying a POV character for consistency',
          scene_id: scene.id,
          scene_title: scene.title
        });
      }

      // Character consistency validation
      if (scene.characters && scene.characters.length > 0) {
        for (const characterRef of scene.characters) {
          const character = characters.find(c => c.id === characterRef || c.name === characterRef);
          if (!character) {
            issues.push({
              type: 'warning',
              category: 'characters',
              message: `Referenced character "${characterRef}" not found in character database`,
              suggestion: 'Add this character to your Story Bible or verify the reference',
              scene_id: scene.id,
              scene_title: scene.title
            });
          }
        }
      }

      // Location consistency validation
      if (scene.location) {
        const location = worldElements.find(w => 
          (w.id === scene.location || w.name === scene.location) && 
          w.element_type === 'location'
        );
        if (!location) {
          issues.push({
            type: 'warning',
            category: 'consistency',
            message: `Referenced location "${scene.location}" not found in worldbuilding database`,
            suggestion: 'Add this location to your Story Bible or verify the reference',
            scene_id: scene.id,
            scene_title: scene.title
          });
        }
      }

      // Structure validation
      if (scene.content.length < 100) {
        issues.push({
          type: 'warning',
          category: 'structure',
          message: 'Scene content is very short',
          suggestion: 'Consider expanding the scene or merging with another scene',
          scene_id: scene.id,
          scene_title: scene.title
        });
      }

      if (!scene.title || scene.title.trim().length === 0) {
        issues.push({
          type: 'error',
          category: 'structure',
          message: 'Scene has no title',
          suggestion: 'Add a descriptive title for better organization',
          scene_id: scene.id,
          scene_title: scene.title || 'Untitled Scene'
        });
      }
    }

    // Timeline validation
    const scenesWithTimeline = scenes.filter(s => s.timeline_position !== undefined);
    if (scenesWithTimeline.length > 1) {
      const sortedScenes = [...scenesWithTimeline].sort((a, b) => 
        (a.timeline_position || 0) - (b.timeline_position || 0)
      );
      
      for (let i = 1; i < sortedScenes.length; i++) {
        const currentPos = sortedScenes[i].timeline_position || 0;
        const prevPos = sortedScenes[i - 1].timeline_position || 0;
        
        if (currentPos === prevPos) {
          issues.push({
            type: 'warning',
            category: 'timeline',
            message: 'Multiple scenes have the same timeline position',
            suggestion: 'Adjust timeline positions to ensure proper sequence',
            scene_id: sortedScenes[i].id,
            scene_title: sortedScenes[i].title
          });
        }
      }
    }

    // Update statistics
    setValidationStats({
      total_scenes: scenes.length,
      total_words: totalWords,
      avg_words_per_scene: scenes.length > 0 ? Math.round(totalWords / scenes.length) : 0,
      scenes_with_targets: scenesWithTargets,
      scenes_meeting_targets: scenesMeetingTargets
    });

    setValidationIssues(issues);
    setIsValidating(false);
    
    if (onValidationComplete) {
      onValidationComplete(issues);
    }
  };

  const countWords = (text: string): number => {
    return text.trim().split(/\s+/).filter(word => word.length > 0).length;
  };

  const getIssueIcon = (type: ValidationIssue['type']) => {
    switch (type) {
      case 'error':
        return <AlertTriangle className="h-4 w-4 text-red-500" />;
      case 'warning':
        return <AlertTriangle className="h-4 w-4 text-yellow-500" />;
      case 'info':
        return <CheckCircle className="h-4 w-4 text-blue-500" />;
      default:
        return <CheckCircle className="h-4 w-4 text-gray-500" />;
    }
  };

  const getIssueColor = (type: ValidationIssue['type']) => {
    switch (type) {
      case 'error':
        return 'border-red-200 bg-red-50';
      case 'warning':
        return 'border-yellow-200 bg-yellow-50';
      case 'info':
        return 'border-blue-200 bg-blue-50';
      default:
        return 'border-gray-200 bg-gray-50';
    }
  };

  const filteredIssues = selectedCategory === 'all' 
    ? validationIssues 
    : validationIssues.filter(issue => issue.category === selectedCategory);

  const issuesByType = {
    error: validationIssues.filter(i => i.type === 'error').length,
    warning: validationIssues.filter(i => i.type === 'warning').length,
    info: validationIssues.filter(i => i.type === 'info').length
  };

  useEffect(() => {
    if (scenes.length > 0) {
      validateScenes();
    }
  }, [scenes, characters, worldElements]);

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold text-gray-900">Scene Validation</h2>
          <p className="text-gray-600">Analyze scenes for consistency, word count targets, and structural issues</p>
        </div>
        <Button 
          onClick={validateScenes}
          disabled={isValidating}
          variant="outline"
        >
          <RefreshCw className={`h-4 w-4 mr-2 ${isValidating ? 'animate-spin' : ''}`} />
          {isValidating ? 'Validating...' : 'Re-validate'}
        </Button>
      </div>

      {/* Statistics */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <Card>
          <CardContent className="p-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-gray-600">Total Scenes</p>
                <p className="text-2xl font-bold">{validationStats.total_scenes}</p>
              </div>
              <MapPin className="h-8 w-8 text-blue-500" />
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-gray-600">Total Words</p>
                <p className="text-2xl font-bold">{validationStats.total_words.toLocaleString()}</p>
              </div>
              <Target className="h-8 w-8 text-green-500" />
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-gray-600">Avg Words/Scene</p>
                <p className="text-2xl font-bold">{validationStats.avg_words_per_scene}</p>
              </div>
              <Target className="h-8 w-8 text-purple-500" />
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-gray-600">Target Compliance</p>
                <p className="text-2xl font-bold">
                  {validationStats.scenes_with_targets > 0 
                    ? Math.round((validationStats.scenes_meeting_targets / validationStats.scenes_with_targets) * 100)
                    : 0}%
                </p>
              </div>
              <CheckCircle className="h-8 w-8 text-green-500" />
            </div>
            {validationStats.scenes_with_targets > 0 && (
              <Progress 
                value={(validationStats.scenes_meeting_targets / validationStats.scenes_with_targets) * 100}
                className="mt-2"
              />
            )}
          </CardContent>
        </Card>
      </div>

      {/* Issue Summary */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <AlertTriangle className="h-5 w-5" />
            Validation Summary
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex items-center gap-4">
            <Badge variant="destructive" className="flex items-center gap-1">
              <AlertTriangle className="h-3 w-3" />
              {issuesByType.error} Errors
            </Badge>
            <Badge variant="secondary" className="flex items-center gap-1 bg-yellow-100 text-yellow-800">
              <AlertTriangle className="h-3 w-3" />
              {issuesByType.warning} Warnings
            </Badge>
            <Badge variant="outline" className="flex items-center gap-1">
              <CheckCircle className="h-3 w-3" />
              {issuesByType.info} Info
            </Badge>
          </div>
        </CardContent>
      </Card>

      {/* Category Filter */}
      <div className="flex flex-wrap gap-2">
        {categories.map(category => {
          const Icon = category.icon;
          const count = category.id === 'all' 
            ? validationIssues.length 
            : validationIssues.filter(i => i.category === category.id).length;
          
          return (
            <Button
              key={category.id}
              variant={selectedCategory === category.id ? 'default' : 'outline'}
              onClick={() => setSelectedCategory(category.id)}
              className="flex items-center gap-2"
            >
              <Icon className="h-4 w-4" />
              {category.label}
              {count > 0 && (
                <Badge variant="secondary" className="ml-1">
                  {count}
                </Badge>
              )}
            </Button>
          );
        })}
      </div>

      {/* Issues List */}
      <div className="space-y-3">
        {filteredIssues.length === 0 ? (
          <Card>
            <CardContent className="p-6 text-center">
              <CheckCircle className="h-12 w-12 text-green-500 mx-auto mb-4" />
              <h3 className="text-lg font-medium text-gray-900 mb-2">
                {selectedCategory === 'all' ? 'No Issues Found!' : `No ${selectedCategory} Issues`}
              </h3>
              <p className="text-gray-600">
                {selectedCategory === 'all' 
                  ? 'All scenes pass validation checks.'
                  : `No issues found in the ${selectedCategory} category.`}
              </p>
            </CardContent>
          </Card>
        ) : (
          filteredIssues.map((issue, index) => (
            <Card key={index} className={getIssueColor(issue.type)}>
              <CardContent className="p-4">
                <div className="flex items-start gap-3">
                  {getIssueIcon(issue.type)}
                  <div className="flex-1">
                    <div className="flex items-center gap-2 mb-1">
                      <h4 className="font-medium text-gray-900">
                        {issue.scene_title}
                      </h4>
                      <Badge variant="outline" className="text-xs">
                        {issue.category}
                      </Badge>
                    </div>
                    <p className="text-gray-700 mb-1">{issue.message}</p>
                    {issue.suggestion && (
                      <p className="text-sm text-gray-600 italic">
                        💡 {issue.suggestion}
                      </p>
                    )}
                  </div>
                </div>
              </CardContent>
            </Card>
          ))
        )}
      </div>
    </div>
  );
};

export default SceneValidation;