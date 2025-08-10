import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Progress } from '@/components/ui/progress';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert';
import { Separator } from '@/components/ui/separator';
import { Switch } from '@/components/ui/switch';
import { Label } from '@/components/ui/label';
import { Input } from '@/components/ui/input';
import { 
  Database, 
  Zap, 
  TrendingUp, 
  Clock, 
  HardDrive, 
  Activity,
  Settings,
  RefreshCw,
  AlertTriangle,
  CheckCircle,
  XCircle,
  BarChart3,
  Trash2,
  Play,
  Pause
} from 'lucide-react';
import { toast } from 'sonner';

interface DatabaseOptimizationStats {
  total_indexes: number;
  unused_indexes: number;
  memory_usage_mb: number;
  cache_hit_rate: number;
  avg_query_time_ms: number;
  total_queries: number;
  slow_queries: number;
}

interface OptimizationConfig {
  enable_auto_indexing: boolean;
  memory_cache_size_mb: number;
  ai_cache_ttl_hours: number;
  cleanup_interval_hours: number;
}

interface OptimizationReport {
  database_stats: DatabaseOptimizationStats;
  recommendations: string[];
  performance_score: number;
  last_optimization: string | null;
}

interface IndexRecommendation {
  table_name: string;
  columns: string[];
  index_type: string;
  estimated_benefit: number;
  reason: string;
}

interface CacheStatistics {
  total_entries: number;
  hit_rate: number;
  miss_rate: number;
  memory_usage_mb: number;
  oldest_entry_age_hours: number;
  newest_entry_age_hours: number;
}

export function OptimizationDashboard() {
  const [stats, setStats] = useState<DatabaseOptimizationStats | null>(null);
  const [config, setConfig] = useState<OptimizationConfig>({
    enable_auto_indexing: true,
    memory_cache_size_mb: 256,
    ai_cache_ttl_hours: 24,
    cleanup_interval_hours: 168, // 1 week
  });
  const [report, setReport] = useState<OptimizationReport | null>(null);
  const [indexRecommendations, setIndexRecommendations] = useState<IndexRecommendation[]>([]);
  const [cacheStats, setCacheStats] = useState<CacheStatistics | null>(null);
  const [isOptimizing, setIsOptimizing] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [activeTab, setActiveTab] = useState('overview');

  useEffect(() => {
    loadOptimizationData();
  }, []);

  const loadOptimizationData = async () => {
    setIsLoading(true);
    try {
      const [statsResult, recommendationsResult, cacheResult] = await Promise.all([
        invoke<DatabaseOptimizationStats>('get_optimization_stats'),
        invoke<IndexRecommendation[]>('get_index_recommendations'),
        invoke<CacheStatistics>('get_cache_statistics')
      ]);
      
      setStats(statsResult);
      setIndexRecommendations(recommendationsResult);
      setCacheStats(cacheResult);
    } catch (error) {
      console.error('Failed to load optimization data:', error);
      toast.error('Failed to load optimization data');
    } finally {
      setIsLoading(false);
    }
  };

  const runOptimization = async () => {
    setIsOptimizing(true);
    try {
      const result = await invoke<OptimizationReport>('run_database_optimization', { config });
      setReport(result);
      setStats(result.database_stats);
      toast.success(`Optimization completed! Performance score: ${result.performance_score.toFixed(1)}`);
      await loadOptimizationData(); // Refresh data
    } catch (error) {
      console.error('Optimization failed:', error);
      toast.error('Optimization failed');
    } finally {
      setIsOptimizing(false);
    }
  };

  const createIndex = async (recommendation: IndexRecommendation) => {
    try {
      await invoke<string>('create_index', {
        table_name: recommendation.table_name,
        columns: recommendation.columns,
        index_type: recommendation.index_type
      });
      toast.success(`Index created for ${recommendation.table_name}`);
      await loadOptimizationData();
    } catch (error) {
      console.error('Failed to create index:', error);
      toast.error('Failed to create index');
    }
  };

  const dropUnusedIndexes = async () => {
    try {
      const droppedIndexes = await invoke<string[]>('drop_unused_indexes', {
        min_usage_threshold: 0.1
      });
      toast.success(`Dropped ${droppedIndexes.length} unused indexes`);
      await loadOptimizationData();
    } catch (error) {
      console.error('Failed to drop unused indexes:', error);
      toast.error('Failed to drop unused indexes');
    }
  };

  const clearAICache = async () => {
    try {
      const clearedCount = await invoke<number>('clear_ai_cache', {
        older_than_hours: config.ai_cache_ttl_hours
      });
      toast.success(`Cleared ${clearedCount} AI cache entries`);
      await loadOptimizationData();
    } catch (error) {
      console.error('Failed to clear AI cache:', error);
      toast.error('Failed to clear AI cache');
    }
  };

  const optimizeMemoryUsage = async () => {
    try {
      await invoke<string>('optimize_memory_usage', {
        target_mb: config.memory_cache_size_mb
      });
      toast.success('Memory usage optimized');
      await loadOptimizationData();
    } catch (error) {
      console.error('Failed to optimize memory:', error);
      toast.error('Failed to optimize memory');
    }
  };

  const getPerformanceScoreColor = (score: number) => {
    if (score >= 90) return 'text-green-600';
    if (score >= 70) return 'text-yellow-600';
    return 'text-red-600';
  };

  const getPerformanceScoreBadge = (score: number) => {
    if (score >= 90) return 'bg-green-100 text-green-800';
    if (score >= 70) return 'bg-yellow-100 text-yellow-800';
    return 'bg-red-100 text-red-800';
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <RefreshCw className="h-8 w-8 animate-spin" />
        <span className="ml-2">Loading optimization data...</span>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Database Optimization</h1>
          <p className="text-muted-foreground">
            Monitor and optimize your StoryWeaver database performance
          </p>
        </div>
        <div className="flex gap-2">
          <Button
            onClick={loadOptimizationData}
            variant="outline"
            size="sm"
            disabled={isLoading}
          >
            <RefreshCw className={`h-4 w-4 mr-2 ${isLoading ? 'animate-spin' : ''}`} />
            Refresh
          </Button>
          <Button
            onClick={runOptimization}
            disabled={isOptimizing}
            className="bg-blue-600 hover:bg-blue-700"
          >
            {isOptimizing ? (
              <>
                <RefreshCw className="h-4 w-4 mr-2 animate-spin" />
                Optimizing...
              </>
            ) : (
              <>
                <Zap className="h-4 w-4 mr-2" />
                Run Optimization
              </>
            )}
          </Button>
        </div>
      </div>

      {/* Performance Score Overview */}
      {report && (
        <Alert className="border-blue-200 bg-blue-50">
          <CheckCircle className="h-4 w-4" />
          <AlertTitle>Optimization Complete</AlertTitle>
          <AlertDescription>
            Performance score: <span className={getPerformanceScoreColor(report.performance_score)}>
              {report.performance_score.toFixed(1)}/100
            </span>
            {report.last_optimization && (
              <span className="ml-2 text-sm text-muted-foreground">
                Last optimized: {new Date(report.last_optimization).toLocaleString()}
              </span>
            )}
          </AlertDescription>
        </Alert>
      )}

      <Tabs value={activeTab} onValueChange={setActiveTab} className="space-y-4">
        <TabsList className="grid w-full grid-cols-5">
          <TabsTrigger value="overview">Overview</TabsTrigger>
          <TabsTrigger value="indexes">Indexes</TabsTrigger>
          <TabsTrigger value="cache">Cache</TabsTrigger>
          <TabsTrigger value="memory">Memory</TabsTrigger>
          <TabsTrigger value="settings">Settings</TabsTrigger>
        </TabsList>

        <TabsContent value="overview" className="space-y-4">
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">Total Indexes</CardTitle>
                <Database className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">{stats?.total_indexes || 0}</div>
                <p className="text-xs text-muted-foreground">
                  {stats?.unused_indexes || 0} unused
                </p>
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">Cache Hit Rate</CardTitle>
                <TrendingUp className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">
                  {((stats?.cache_hit_rate || 0) * 100).toFixed(1)}%
                </div>
                <Progress value={(stats?.cache_hit_rate || 0) * 100} className="mt-2" />
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">Avg Query Time</CardTitle>
                <Clock className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">
                  {(stats?.avg_query_time_ms || 0).toFixed(1)}ms
                </div>
                <p className="text-xs text-muted-foreground">
                  {stats?.slow_queries || 0} slow queries
                </p>
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">Memory Usage</CardTitle>
                <HardDrive className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">{stats?.memory_usage_mb || 0}MB</div>
                <Progress 
                  value={Math.min((stats?.memory_usage_mb || 0) / 512 * 100, 100)} 
                  className="mt-2" 
                />
              </CardContent>
            </Card>
          </div>

          {/* Recommendations */}
          {report && report.recommendations.length > 0 && (
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Activity className="h-5 w-5" />
                  Optimization Recommendations
                </CardTitle>
                <CardDescription>
                  Suggestions to improve your database performance
                </CardDescription>
              </CardHeader>
              <CardContent>
                <div className="space-y-2">
                  {report.recommendations.map((recommendation, index) => (
                    <div key={index} className="flex items-start gap-2 p-3 bg-muted rounded-lg">
                      <AlertTriangle className="h-4 w-4 text-yellow-600 mt-0.5 flex-shrink-0" />
                      <span className="text-sm">{recommendation}</span>
                    </div>
                  ))}
                </div>
              </CardContent>
            </Card>
          )}
        </TabsContent>

        <TabsContent value="indexes" className="space-y-4">
          <div className="flex items-center justify-between">
            <div>
              <h3 className="text-lg font-medium">Index Management</h3>
              <p className="text-sm text-muted-foreground">
                Manage database indexes for optimal query performance
              </p>
            </div>
            <Button
              onClick={dropUnusedIndexes}
              variant="outline"
              className="text-red-600 hover:text-red-700"
              disabled={!stats?.unused_indexes}
            >
              <Trash2 className="h-4 w-4 mr-2" />
              Drop Unused ({stats?.unused_indexes || 0})
            </Button>
          </div>

          <div className="grid gap-4">
            {indexRecommendations.map((recommendation, index) => (
              <Card key={index}>
                <CardHeader>
                  <div className="flex items-center justify-between">
                    <CardTitle className="text-base">
                      {recommendation.table_name}
                    </CardTitle>
                    <Badge className={getPerformanceScoreBadge(recommendation.estimated_benefit * 100)}>
                      {(recommendation.estimated_benefit * 100).toFixed(0)}% benefit
                    </Badge>
                  </div>
                  <CardDescription>{recommendation.reason}</CardDescription>
                </CardHeader>
                <CardContent>
                  <div className="flex items-center justify-between">
                    <div>
                      <p className="text-sm font-medium">Columns: {recommendation.columns.join(', ')}</p>
                      <p className="text-sm text-muted-foreground">Type: {recommendation.index_type}</p>
                    </div>
                    <Button
                      onClick={() => createIndex(recommendation)}
                      size="sm"
                    >
                      <Play className="h-4 w-4 mr-2" />
                      Create Index
                    </Button>
                  </div>
                </CardContent>
              </Card>
            ))}
            
            {indexRecommendations.length === 0 && (
              <Card>
                <CardContent className="flex items-center justify-center py-8">
                  <div className="text-center">
                    <CheckCircle className="h-8 w-8 text-green-600 mx-auto mb-2" />
                    <p className="text-sm text-muted-foreground">
                      No index recommendations at this time
                    </p>
                  </div>
                </CardContent>
              </Card>
            )}
          </div>
        </TabsContent>

        <TabsContent value="cache" className="space-y-4">
          <div className="flex items-center justify-between">
            <div>
              <h3 className="text-lg font-medium">Cache Management</h3>
              <p className="text-sm text-muted-foreground">
                Monitor and manage AI response caching
              </p>
            </div>
            <Button
              onClick={clearAICache}
              variant="outline"
              className="text-red-600 hover:text-red-700"
            >
              <Trash2 className="h-4 w-4 mr-2" />
              Clear AI Cache
            </Button>
          </div>

          {cacheStats && (
            <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
              <Card>
                <CardHeader className="pb-2">
                  <CardTitle className="text-sm font-medium">Total Entries</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="text-2xl font-bold">{cacheStats.total_entries}</div>
                </CardContent>
              </Card>

              <Card>
                <CardHeader className="pb-2">
                  <CardTitle className="text-sm font-medium">Hit Rate</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="text-2xl font-bold">
                    {(cacheStats.hit_rate * 100).toFixed(1)}%
                  </div>
                  <Progress value={cacheStats.hit_rate * 100} className="mt-2" />
                </CardContent>
              </Card>

              <Card>
                <CardHeader className="pb-2">
                  <CardTitle className="text-sm font-medium">Memory Usage</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="text-2xl font-bold">{cacheStats.memory_usage_mb}MB</div>
                </CardContent>
              </Card>
            </div>
          )}
        </TabsContent>

        <TabsContent value="memory" className="space-y-4">
          <div className="flex items-center justify-between">
            <div>
              <h3 className="text-lg font-medium">Memory Optimization</h3>
              <p className="text-sm text-muted-foreground">
                Optimize memory usage for better performance
              </p>
            </div>
            <Button onClick={optimizeMemoryUsage} variant="outline">
              <Zap className="h-4 w-4 mr-2" />
              Optimize Memory
            </Button>
          </div>

          <Card>
            <CardHeader>
              <CardTitle>Memory Statistics</CardTitle>
              <CardDescription>
                Current memory usage and optimization status
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                <div>
                  <div className="flex justify-between text-sm mb-1">
                    <span>Database Memory</span>
                    <span>{stats?.memory_usage_mb || 0}MB</span>
                  </div>
                  <Progress value={Math.min((stats?.memory_usage_mb || 0) / 512 * 100, 100)} />
                </div>
                
                {cacheStats && (
                  <div>
                    <div className="flex justify-between text-sm mb-1">
                      <span>Cache Memory</span>
                      <span>{cacheStats.memory_usage_mb}MB</span>
                    </div>
                    <Progress value={Math.min(cacheStats.memory_usage_mb / 256 * 100, 100)} />
                  </div>
                )}
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="settings" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Settings className="h-5 w-5" />
                Optimization Settings
              </CardTitle>
              <CardDescription>
                Configure automatic optimization behavior
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-6">
              <div className="flex items-center justify-between">
                <div className="space-y-0.5">
                  <Label htmlFor="auto-indexing">Auto Indexing</Label>
                  <p className="text-sm text-muted-foreground">
                    Automatically create recommended indexes
                  </p>
                </div>
                <Switch
                  id="auto-indexing"
                  checked={config.enable_auto_indexing}
                  onCheckedChange={(checked) => 
                    setConfig(prev => ({ ...prev, enable_auto_indexing: checked }))
                  }
                />
              </div>

              <Separator />

              <div className="space-y-2">
                <Label htmlFor="memory-cache-size">Memory Cache Size (MB)</Label>
                <Input
                  id="memory-cache-size"
                  type="number"
                  value={config.memory_cache_size_mb}
                  onChange={(e) => 
                    setConfig(prev => ({ 
                      ...prev, 
                      memory_cache_size_mb: parseInt(e.target.value) || 256 
                    }))
                  }
                  min="64"
                  max="1024"
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="ai-cache-ttl">AI Cache TTL (Hours)</Label>
                <Input
                  id="ai-cache-ttl"
                  type="number"
                  value={config.ai_cache_ttl_hours}
                  onChange={(e) => 
                    setConfig(prev => ({ 
                      ...prev, 
                      ai_cache_ttl_hours: parseInt(e.target.value) || 24 
                    }))
                  }
                  min="1"
                  max="168"
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="cleanup-interval">Cleanup Interval (Hours)</Label>
                <Input
                  id="cleanup-interval"
                  type="number"
                  value={config.cleanup_interval_hours}
                  onChange={(e) => 
                    setConfig(prev => ({ 
                      ...prev, 
                      cleanup_interval_hours: parseInt(e.target.value) || 168 
                    }))
                  }
                  min="24"
                  max="720"
                />
              </div>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  );
}

export default OptimizationDashboard;