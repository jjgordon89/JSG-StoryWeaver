import React, { useState, useEffect } from 'react';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '../../../../components/ui/tabs';
import { Card, CardContent, CardHeader, CardTitle } from '../../../../components/ui/Card';
import { Button } from '../../../../components/ui/Button';
import BraindumpEditor from './BraindumpEditor';
import CharactersManager from './CharactersManager';
import WorldbuildingManager from './WorldbuildingManager';
import OutlineManager from './OutlineManager';
import ScenesManager from './ScenesManager';
import { useStoryBible } from '../../hooks/useStoryBible';
import type { StoryBibleProps } from '../../../../types/storyBible';

export const StoryBible: React.FC<StoryBibleProps> = ({ projectId, seriesId }) => {
  const [activeTab, setActiveTab] = useState<'braindump' | 'characters' | 'worldbuilding' | 'outline' | 'scenes'>('braindump');
  const { storyBible, isLoading, error, loadStoryBible, clearError } = useStoryBible();

  useEffect(() => {
    if (projectId) {
      loadStoryBible(projectId);
    }
  }, [projectId, loadStoryBible]);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
        <span className="ml-2 text-slate-600">Loading Story Bible...</span>
      </div>
    );
  }

  if (error) {
    return (
      <Card className="m-4">
        <CardContent className="p-6">
          <div className="text-red-600 mb-4">
            <h3 className="font-semibold mb-2">Error Loading Story Bible</h3>
            <p>{error}</p>
          </div>
          <Button onClick={clearError} variant="outline">
            Dismiss
          </Button>
        </CardContent>
      </Card>
    );
  }

  return (
    <div className="story-bible h-full flex flex-col bg-slate-50 dark:bg-slate-900">
      {/* Header */}
      <div className="border-b border-slate-200 dark:border-slate-700 bg-white dark:bg-slate-800 px-6 py-4">
        <div className="flex justify-between items-start">
          <div>
            <h1 className="text-2xl font-bold text-slate-900 dark:text-slate-100 mb-1">
              Story Bible
            </h1>
            <p className="text-slate-600 dark:text-slate-400 text-sm">
              Manage your story's world, characters, and narrative structure
            </p>
          </div>
          <div className="flex gap-2">
            <Button variant="outline" size="sm">
              Export
            </Button>
            <Button variant="outline" size="sm">
              Import
            </Button>
          </div>
        </div>
      </div>

      {/* Tabs Navigation */}
      <Tabs value={activeTab} onValueChange={(value) => setActiveTab(value as any)} className="flex-1 flex flex-col">
        <div className="border-b border-slate-200 dark:border-slate-700 bg-white dark:bg-slate-800 px-6">
          <TabsList className="grid w-full grid-cols-5">
            <TabsTrigger value="braindump" className="flex items-center gap-2">
              <span>🧠</span>
              Braindump
            </TabsTrigger>
            <TabsTrigger value="characters" className="flex items-center gap-2">
              <span>👥</span>
              Characters
            </TabsTrigger>
            <TabsTrigger value="worldbuilding" className="flex items-center gap-2">
              <span>🌍</span>
              Worldbuilding
            </TabsTrigger>
            <TabsTrigger value="outline" className="flex items-center gap-2">
              <span>📋</span>
              Outline
            </TabsTrigger>
            <TabsTrigger value="scenes" className="flex items-center gap-2">
              <span>🎬</span>
              Scenes
            </TabsTrigger>
          </TabsList>
        </div>

        {/* Tab Content */}
        <div className="flex-1 overflow-hidden">
          <TabsContent value="braindump" className="h-full m-0">
            <BraindumpEditor projectId={projectId} />
          </TabsContent>
          
          <TabsContent value="characters" className="h-full m-0">
            <CharactersManager projectId={projectId} seriesId={seriesId} />
          </TabsContent>
          
          <TabsContent value="worldbuilding" className="h-full m-0">
            <WorldbuildingManager projectId={projectId} seriesId={seriesId} />
          </TabsContent>
          
          <TabsContent value="outline" className="h-full m-0">
            <OutlineManager projectId={projectId} />
          </TabsContent>
          
          <TabsContent value="scenes" className="h-full m-0">
            <ScenesManager outlineId="" />
          </TabsContent>
        </div>
      </Tabs>
    </div>
  );
};

export default StoryBible;