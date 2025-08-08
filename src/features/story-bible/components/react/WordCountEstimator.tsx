import React, { useState, useEffect } from 'react';
import { Button } from '../../../../components/ui/Button';
import { Card, CardContent, CardHeader, CardTitle } from '../../../../components/ui/Card';
import { Input } from '../../../../components/ui/input';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../../../../components/ui/select';
import { Badge } from '../../../../components/ui/badge';
import { Progress } from '../../../../components/ui/progress';
import { Calculator, Target, TrendingUp, Clock, BookOpen, FileText, BarChart3 } from 'lucide-react';

interface Scene {
  id: string;
  title: string;
  content: string;
  target_word_count?: number;
  chapter_id?: string;
  estimated_duration?: number; // in minutes
}

interface Chapter {
  id: string;
  title: string;
  target_word_count?: number;
  scenes?: Scene[];
}

interface WordCountEstimatorProps {
  scenes: Scene[];
  chapters?: Chapter[];
  projectTargetWords?: number;
  onEstimateUpdate?: (estimates: ProjectEstimates) => void;
}

interface ProjectEstimates {
  current_word_count: number;
  target_word_count: number;
  completion_percentage: number;
  estimated_remaining_words: number;
  estimated_completion_time: number; // in hours
  daily_word_goal: number;
  estimated_days_to_completion: number;
  scene_estimates: SceneEstimate[];
  chapter_estimates: ChapterEstimate[];
}

interface SceneEstimate {
  scene_id: string;
  scene_title: string;
  current_words: number;
  target_words: number;
  completion_percentage: number;
  estimated_reading_time: number; // in minutes
  estimated_writing_time: number; // in minutes
  status: 'not_started' | 'in_progress' | 'completed' | 'over_target';
}

interface ChapterEstimate {
  chapter_id: string;
  chapter_title: string;
  current_words: number;
  target_words: number;
  completion_percentage: number;
  scene_count: number;
  completed_scenes: number;
}

const WordCountEstimator: React.FC<WordCountEstimatorProps> = ({
  scenes,
  chapters = [],
  projectTargetWords = 80000,
  onEstimateUpdate
}) => {
  const [estimates, setEstimates] = useState<ProjectEstimates | null>(null);
  const [writingSpeed, setWritingSpeed] = useState(500); // words per hour
  const [dailyWritingTime, setDailyWritingTime] = useState(2); // hours per day
  const [targetGenre, setTargetGenre] = useState('novel');
  const [customTarget, setCustomTarget] = useState(projectTargetWords);

  const genreTargets = {
    'short_story': { min: 1000, max: 7500, typical: 4000 },
    'novelette': { min: 7500, max: 17500, typical: 12500 },
    'novella': { min: 17500, max: 40000, typical: 30000 },
    'novel': { min: 40000, max: 120000, typical: 80000 },
    'epic_fantasy': { min: 80000, max: 200000, typical: 120000 },
    'romance': { min: 50000, max: 90000, typical: 70000 },
    'mystery': { min: 70000, max: 90000, typical: 80000 },
    'sci_fi': { min: 80000, max: 120000, typical: 100000 },
    'young_adult': { min: 50000, max: 80000, typical: 65000 },
    'middle_grade': { min: 20000, max: 50000, typical: 35000 }
  };

  const calculateEstimates = () => {
    const currentWordCount = scenes.reduce((total, scene) => {
      return total + countWords(scene.content);
    }, 0);

    const targetWordCount = customTarget;
    const completionPercentage = targetWordCount > 0 ? (currentWordCount / targetWordCount) * 100 : 0;
    const remainingWords = Math.max(0, targetWordCount - currentWordCount);
    
    // Calculate scene estimates
    const sceneEstimates: SceneEstimate[] = scenes.map(scene => {
      const currentWords = countWords(scene.content);
      const targetWords = scene.target_word_count || 2000; // Default scene target
      const sceneCompletion = targetWords > 0 ? (currentWords / targetWords) * 100 : 0;
      
      let status: SceneEstimate['status'] = 'not_started';
      if (currentWords === 0) {
        status = 'not_started';
      } else if (sceneCompletion >= 100) {
        status = currentWords > targetWords * 1.2 ? 'over_target' : 'completed';
      } else {
        status = 'in_progress';
      }

      return {
        scene_id: scene.id,
        scene_title: scene.title,
        current_words: currentWords,
        target_words: targetWords,
        completion_percentage: Math.min(sceneCompletion, 100),
        estimated_reading_time: Math.ceil(currentWords / 250), // 250 words per minute reading speed
        estimated_writing_time: Math.ceil((targetWords - currentWords) / (writingSpeed / 60)), // minutes
        status
      };
    });

    // Calculate chapter estimates
    const chapterEstimates: ChapterEstimate[] = chapters.map(chapter => {
      const chapterScenes = scenes.filter(scene => scene.chapter_id === chapter.id);
      const currentWords = chapterScenes.reduce((total, scene) => total + countWords(scene.content), 0);
      const targetWords = chapter.target_word_count || (chapterScenes.length * 2000);
      const completedScenes = chapterScenes.filter(scene => {
        const sceneWords = countWords(scene.content);
        const sceneTarget = scene.target_word_count || 2000;
        return sceneWords >= sceneTarget * 0.9; // 90% of target considered complete
      }).length;

      return {
        chapter_id: chapter.id,
        chapter_title: chapter.title,
        current_words: currentWords,
        target_words: targetWords,
        completion_percentage: targetWords > 0 ? (currentWords / targetWords) * 100 : 0,
        scene_count: chapterScenes.length,
        completed_scenes: completedScenes
      };
    });

    // Calculate time estimates
    const estimatedCompletionTime = remainingWords / writingSpeed; // hours
    const dailyWordGoal = dailyWritingTime * writingSpeed;
    const estimatedDaysToCompletion = dailyWordGoal > 0 ? Math.ceil(remainingWords / dailyWordGoal) : 0;

    const projectEstimates: ProjectEstimates = {
      current_word_count: currentWordCount,
      target_word_count: targetWordCount,
      completion_percentage: Math.min(completionPercentage, 100),
      estimated_remaining_words: remainingWords,
      estimated_completion_time: estimatedCompletionTime,
      daily_word_goal: dailyWordGoal,
      estimated_days_to_completion: estimatedDaysToCompletion,
      scene_estimates: sceneEstimates,
      chapter_estimates: chapterEstimates
    };

    setEstimates(projectEstimates);
    
    if (onEstimateUpdate) {
      onEstimateUpdate(projectEstimates);
    }
  };

  const countWords = (text: string): number => {
    return text.trim().split(/\s+/).filter(word => word.length > 0).length;
  };

  const formatTime = (hours: number): string => {
    if (hours < 1) {
      return `${Math.round(hours * 60)} minutes`;
    } else if (hours < 24) {
      return `${hours.toFixed(1)} hours`;
    } else {
      const days = Math.floor(hours / 24);
      const remainingHours = hours % 24;
      return `${days} days, ${remainingHours.toFixed(1)} hours`;
    }
  };

  const getStatusColor = (status: SceneEstimate['status']) => {
    switch (status) {
      case 'completed':
        return 'bg-green-100 text-green-800';
      case 'in_progress':
        return 'bg-blue-100 text-blue-800';
      case 'over_target':
        return 'bg-orange-100 text-orange-800';
      case 'not_started':
      default:
        return 'bg-gray-100 text-gray-800';
    }
  };

  const getStatusLabel = (status: SceneEstimate['status']) => {
    switch (status) {
      case 'completed':
        return 'Complete';
      case 'in_progress':
        return 'In Progress';
      case 'over_target':
        return 'Over Target';
      case 'not_started':
      default:
        return 'Not Started';
    }
  };

  useEffect(() => {
    calculateEstimates();
  }, [scenes, chapters, writingSpeed, dailyWritingTime, customTarget]);

  useEffect(() => {
    if (targetGenre in genreTargets) {
      setCustomTarget(genreTargets[targetGenre as keyof typeof genreTargets].typical);
    }
  }, [targetGenre]);

  if (!estimates) {
    return (
      <div className="flex items-center justify-center p-8">
        <div className="text-center">
          <Calculator className="h-12 w-12 text-gray-400 mx-auto mb-4" />
          <p className="text-gray-600">Calculating estimates...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold text-gray-900">Word Count Estimator</h2>
          <p className="text-gray-600">Track progress and estimate completion times</p>
        </div>
        <Button onClick={calculateEstimates} variant="outline">
          <Calculator className="h-4 w-4 mr-2" />
          Recalculate
        </Button>
      </div>

      {/* Settings */}
      <Card>
        <CardHeader>
          <CardTitle>Estimation Settings</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Target Genre
              </label>
              <Select value={targetGenre} onValueChange={setTargetGenre}>
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="short_story">Short Story (1K-7.5K)</SelectItem>
                  <SelectItem value="novelette">Novelette (7.5K-17.5K)</SelectItem>
                  <SelectItem value="novella">Novella (17.5K-40K)</SelectItem>
                  <SelectItem value="novel">Novel (40K-120K)</SelectItem>
                  <SelectItem value="epic_fantasy">Epic Fantasy (80K-200K)</SelectItem>
                  <SelectItem value="romance">Romance (50K-90K)</SelectItem>
                  <SelectItem value="mystery">Mystery (70K-90K)</SelectItem>
                  <SelectItem value="sci_fi">Sci-Fi (80K-120K)</SelectItem>
                  <SelectItem value="young_adult">Young Adult (50K-80K)</SelectItem>
                  <SelectItem value="middle_grade">Middle Grade (20K-50K)</SelectItem>
                </SelectContent>
              </Select>
            </div>
            
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Target Word Count
              </label>
              <Input
                type="number"
                value={customTarget}
                onChange={(e) => setCustomTarget(Number(e.target.value))}
                min="1000"
                step="1000"
              />
            </div>
            
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Writing Speed (words/hour)
              </label>
              <Input
                type="number"
                value={writingSpeed}
                onChange={(e) => setWritingSpeed(Number(e.target.value))}
                min="100"
                step="50"
              />
            </div>
            
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Daily Writing Time (hours)
              </label>
              <Input
                type="number"
                value={dailyWritingTime}
                onChange={(e) => setDailyWritingTime(Number(e.target.value))}
                min="0.5"
                step="0.5"
              />
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Overall Progress */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <Card>
          <CardContent className="p-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-gray-600">Current Words</p>
                <p className="text-2xl font-bold">{estimates.current_word_count.toLocaleString()}</p>
              </div>
              <FileText className="h-8 w-8 text-blue-500" />
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-gray-600">Target Words</p>
                <p className="text-2xl font-bold">{estimates.target_word_count.toLocaleString()}</p>
              </div>
              <Target className="h-8 w-8 text-green-500" />
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-gray-600">Progress</p>
                <p className="text-2xl font-bold">{estimates.completion_percentage.toFixed(1)}%</p>
              </div>
              <TrendingUp className="h-8 w-8 text-purple-500" />
            </div>
            <Progress value={estimates.completion_percentage} className="mt-2" />
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-gray-600">Est. Completion</p>
                <p className="text-lg font-bold">{estimates.estimated_days_to_completion} days</p>
              </div>
              <Clock className="h-8 w-8 text-orange-500" />
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Detailed Estimates */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card>
          <CardHeader>
            <CardTitle>Writing Goals</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              <div className="flex justify-between items-center">
                <span className="text-gray-600">Daily Word Goal:</span>
                <span className="font-semibold">{estimates.daily_word_goal.toLocaleString()} words</span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-gray-600">Remaining Words:</span>
                <span className="font-semibold">{estimates.estimated_remaining_words.toLocaleString()} words</span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-gray-600">Estimated Time:</span>
                <span className="font-semibold">{formatTime(estimates.estimated_completion_time)}</span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-gray-600">Days to Completion:</span>
                <span className="font-semibold">{estimates.estimated_days_to_completion} days</span>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Project Statistics</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              <div className="flex justify-between items-center">
                <span className="text-gray-600">Total Scenes:</span>
                <span className="font-semibold">{estimates.scene_estimates.length}</span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-gray-600">Completed Scenes:</span>
                <span className="font-semibold">
                  {estimates.scene_estimates.filter(s => s.status === 'completed').length}
                </span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-gray-600">Average Scene Length:</span>
                <span className="font-semibold">
                  {estimates.scene_estimates.length > 0 
                    ? Math.round(estimates.current_word_count / estimates.scene_estimates.length)
                    : 0} words
                </span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-gray-600">Reading Time:</span>
                <span className="font-semibold">
                  {Math.ceil(estimates.current_word_count / 250)} minutes
                </span>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Scene Progress */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <BarChart3 className="h-5 w-5" />
            Scene Progress
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-3 max-h-96 overflow-y-auto">
            {estimates.scene_estimates.map(scene => (
              <div key={scene.scene_id} className="border rounded-lg p-3">
                <div className="flex items-center justify-between mb-2">
                  <h4 className="font-medium text-gray-900">{scene.scene_title}</h4>
                  <Badge className={getStatusColor(scene.status)}>
                    {getStatusLabel(scene.status)}
                  </Badge>
                </div>
                <div className="flex items-center justify-between text-sm text-gray-600 mb-2">
                  <span>{scene.current_words.toLocaleString()} / {scene.target_words.toLocaleString()} words</span>
                  <span>{scene.completion_percentage.toFixed(1)}%</span>
                </div>
                <Progress value={scene.completion_percentage} className="mb-2" />
                <div className="flex justify-between text-xs text-gray-500">
                  <span>Reading: {scene.estimated_reading_time} min</span>
                  <span>Writing: {scene.estimated_writing_time} min remaining</span>
                </div>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>

      {/* Chapter Progress */}
      {estimates.chapter_estimates.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <BookOpen className="h-5 w-5" />
              Chapter Progress
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-3">
              {estimates.chapter_estimates.map(chapter => (
                <div key={chapter.chapter_id} className="border rounded-lg p-3">
                  <div className="flex items-center justify-between mb-2">
                    <h4 className="font-medium text-gray-900">{chapter.chapter_title}</h4>
                    <span className="text-sm text-gray-600">
                      {chapter.completed_scenes} / {chapter.scene_count} scenes
                    </span>
                  </div>
                  <div className="flex items-center justify-between text-sm text-gray-600 mb-2">
                    <span>{chapter.current_words.toLocaleString()} / {chapter.target_words.toLocaleString()} words</span>
                    <span>{chapter.completion_percentage.toFixed(1)}%</span>
                  </div>
                  <Progress value={chapter.completion_percentage} />
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
};

export default WordCountEstimator;