import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { 
  CreditCard, 
  TrendingUp, 
  TrendingDown, 
  AlertTriangle, 
  CheckCircle, 
  Settings, 
  RefreshCw, 
  DollarSign, 
  Calendar, 
  BarChart3,
  Zap,
  Clock,
  Target
} from 'lucide-react';
import { Button } from '../ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '../ui/card';
import { Progress } from '../ui/progress';
import { Badge } from '../ui/badge';
import { Separator } from '../ui/separator';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../ui/select';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '../ui/tabs';
import { useAICredits } from '../../hooks/useAI';

interface AICreditManagerProps {
  className?: string;
}

interface UsageData {
  date: string;
  credits: number;
  cost: number;
  operations: number;
}

interface ProviderUsage {
  provider: string;
  credits: number;
  percentage: number;
  cost: number;
  color: string;
}

export const AICreditManager: React.FC<AICreditManagerProps> = ({ className = '' }) => {
  const [selectedPeriod, setSelectedPeriod] = useState('7d');
  const [showSettings, setShowSettings] = useState(false);
  
  const { 
    credits, 
    usage, 
    providers, 
    refreshCredits, 
    updateCreditLimit, 
    setLowCreditAlert,
    getUsageHistory,
    getProviderUsage,
    estimateCredits
  } = useAICredits();
  
  // Mock data for demonstration - in real app, this would come from the store
  const [usageHistory] = useState<UsageData[]>([
    { date: '2024-01-01', credits: 150, cost: 1.50, operations: 25 },
    { date: '2024-01-02', credits: 200, cost: 2.00, operations: 32 },
    { date: '2024-01-03', credits: 180, cost: 1.80, operations: 28 },
    { date: '2024-01-04', credits: 220, cost: 2.20, operations: 35 },
    { date: '2024-01-05', credits: 160, cost: 1.60, operations: 24 },
    { date: '2024-01-06', credits: 190, cost: 1.90, operations: 30 },
    { date: '2024-01-07', credits: 210, cost: 2.10, operations: 33 }
  ]);
  
  const [providerUsage] = useState<ProviderUsage[]>([
    { provider: 'OpenAI', credits: 850, percentage: 45, cost: 8.50, color: 'green' },
    { provider: 'Claude', credits: 650, percentage: 35, cost: 6.50, color: 'blue' },
    { provider: 'Gemini', credits: 380, percentage: 20, cost: 3.80, color: 'purple' }
  ]);
  
  const totalUsedCredits = usageHistory.reduce((sum, day) => sum + day.credits, 0);
  const totalCost = usageHistory.reduce((sum, day) => sum + day.cost, 0);
  const averageDaily = totalUsedCredits / usageHistory.length;
  const projectedMonthly = averageDaily * 30;
  
  const creditUtilization = (credits.used / credits.total) * 100;
  const isLowCredits = credits.remaining < credits.alertThreshold;
  const daysRemaining = Math.floor(credits.remaining / averageDaily);
  
  const getStatusColor = () => {
    if (creditUtilization >= 90) return 'red';
    if (creditUtilization >= 75) return 'yellow';
    return 'green';
  };
  
  const getStatusIcon = () => {
    if (creditUtilization >= 90) return AlertTriangle;
    if (creditUtilization >= 75) return TrendingUp;
    return CheckCircle;
  };
  
  const StatusIcon = getStatusIcon();
  const statusColor = getStatusColor();
  
  return (
    <div className={`space-y-6 ${className}`}>
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <CreditCard className="w-6 h-6 text-blue-500" />
          <h2 className="text-xl font-semibold">AI Credit Manager</h2>
        </div>
        
        <div className="flex items-center gap-2">
          <Button variant="outline" size="sm" onClick={refreshCredits}>
            <RefreshCw className="w-4 h-4 mr-1" />
            Refresh
          </Button>
          <Button variant="outline" size="sm" onClick={() => setShowSettings(!showSettings)}>
            <Settings className="w-4 h-4" />
          </Button>
        </div>
      </div>
      
      {/* Credit Overview */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        {/* Current Balance */}
        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-sm font-medium flex items-center gap-2">
              <StatusIcon className={`w-4 h-4 text-${statusColor}-500`} />
              Current Balance
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-3">
              <div className="text-2xl font-bold">
                {credits.remaining.toLocaleString()}
                <span className="text-sm font-normal text-gray-500 ml-1">credits</span>
              </div>
              
              <Progress 
                value={creditUtilization} 
                className={`h-2 bg-${statusColor}-100`}
              />
              
              <div className="flex justify-between text-xs text-gray-600 dark:text-gray-400">
                <span>{credits.used.toLocaleString()} used</span>
                <span>{credits.total.toLocaleString()} total</span>
              </div>
              
              {isLowCredits && (
                <div className="flex items-center gap-1 text-xs text-amber-600 dark:text-amber-400">
                  <AlertTriangle className="w-3 h-3" />
                  Low credit warning
                </div>
              )}
            </div>
          </CardContent>
        </Card>
        
        {/* Usage Stats */}
        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-sm font-medium flex items-center gap-2">
              <BarChart3 className="w-4 h-4 text-purple-500" />
              Usage Stats
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-3">
              <div className="flex items-center justify-between">
                <span className="text-sm text-gray-600 dark:text-gray-400">Daily Average</span>
                <span className="font-medium">{Math.round(averageDaily)} credits</span>
              </div>
              
              <div className="flex items-center justify-between">
                <span className="text-sm text-gray-600 dark:text-gray-400">This Week</span>
                <span className="font-medium">{totalUsedCredits.toLocaleString()} credits</span>
              </div>
              
              <div className="flex items-center justify-between">
                <span className="text-sm text-gray-600 dark:text-gray-400">Projected Monthly</span>
                <span className="font-medium">{Math.round(projectedMonthly).toLocaleString()} credits</span>
              </div>
              
              <div className="flex items-center justify-between">
                <span className="text-sm text-gray-600 dark:text-gray-400">Days Remaining</span>
                <Badge variant={daysRemaining < 7 ? 'destructive' : 'secondary'}>
                  ~{daysRemaining} days
                </Badge>
              </div>
            </div>
          </CardContent>
        </Card>
        
        {/* Cost Overview */}
        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-sm font-medium flex items-center gap-2">
              <DollarSign className="w-4 h-4 text-green-500" />
              Cost Overview
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-3">
              <div className="text-2xl font-bold">
                ${totalCost.toFixed(2)}
                <span className="text-sm font-normal text-gray-500 ml-1">this week</span>
              </div>
              
              <div className="flex items-center justify-between">
                <span className="text-sm text-gray-600 dark:text-gray-400">Cost per Credit</span>
                <span className="font-medium">$0.01</span>
              </div>
              
              <div className="flex items-center justify-between">
                <span className="text-sm text-gray-600 dark:text-gray-400">Projected Monthly</span>
                <span className="font-medium">${(totalCost * 4.3).toFixed(2)}</span>
              </div>
              
              <div className="flex items-center gap-1 text-xs">
                <TrendingDown className="w-3 h-3 text-green-500" />
                <span className="text-green-600 dark:text-green-400">12% lower than last week</span>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
      
      {/* Detailed Analytics */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle className="flex items-center gap-2">
              <BarChart3 className="w-5 h-5" />
              Usage Analytics
            </CardTitle>
            
            <Select value={selectedPeriod} onValueChange={setSelectedPeriod}>
              <SelectTrigger className="w-32">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="7d">Last 7 days</SelectItem>
                <SelectItem value="30d">Last 30 days</SelectItem>
                <SelectItem value="90d">Last 90 days</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </CardHeader>
        <CardContent>
          <Tabs defaultValue="usage" className="space-y-4">
            <TabsList>
              <TabsTrigger value="usage">Daily Usage</TabsTrigger>
              <TabsTrigger value="providers">By Provider</TabsTrigger>
              <TabsTrigger value="operations">Operations</TabsTrigger>
            </TabsList>
            
            <TabsContent value="usage" className="space-y-4">
              {/* Usage Chart Placeholder */}
              <div className="h-48 bg-gray-50 dark:bg-gray-800 rounded-lg flex items-center justify-center">
                <div className="text-center text-gray-500 dark:text-gray-400">
                  <BarChart3 className="w-8 h-8 mx-auto mb-2" />
                  <p className="text-sm">Usage chart would be rendered here</p>
                  <p className="text-xs">Integration with charting library needed</p>
                </div>
              </div>
              
              {/* Daily Breakdown */}
              <div className="space-y-2">
                <h4 className="font-medium text-sm">Daily Breakdown</h4>
                <div className="space-y-2">
                  {usageHistory.map((day, index) => (
                    <div key={day.date} className="flex items-center justify-between p-2 bg-gray-50 dark:bg-gray-800 rounded">
                      <div className="flex items-center gap-3">
                        <Calendar className="w-4 h-4 text-gray-400" />
                        <span className="text-sm">
                          {new Date(day.date).toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric' })}
                        </span>
                      </div>
                      
                      <div className="flex items-center gap-4 text-sm">
                        <div className="flex items-center gap-1">
                          <Zap className="w-3 h-3 text-blue-500" />
                          {day.credits} credits
                        </div>
                        <div className="flex items-center gap-1">
                          <DollarSign className="w-3 h-3 text-green-500" />
                          ${day.cost.toFixed(2)}
                        </div>
                        <div className="flex items-center gap-1">
                          <Target className="w-3 h-3 text-purple-500" />
                          {day.operations} ops
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            </TabsContent>
            
            <TabsContent value="providers" className="space-y-4">
              <div className="space-y-4">
                {providerUsage.map((provider) => (
                  <div key={provider.provider} className="space-y-2">
                    <div className="flex items-center justify-between">
                      <div className="flex items-center gap-2">
                        <div className={`w-3 h-3 rounded-full bg-${provider.color}-500`} />
                        <span className="font-medium">{provider.provider}</span>
                      </div>
                      
                      <div className="flex items-center gap-4 text-sm">
                        <span>{provider.credits.toLocaleString()} credits</span>
                        <span className="text-gray-500">${provider.cost.toFixed(2)}</span>
                        <Badge variant="outline">{provider.percentage}%</Badge>
                      </div>
                    </div>
                    
                    <Progress value={provider.percentage} className="h-2" />
                  </div>
                ))}
              </div>
            </TabsContent>
            
            <TabsContent value="operations" className="space-y-4">
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                {[
                  { name: 'Auto Write', count: 45, credits: 450, icon: PenTool },
                  { name: 'Rewrite', count: 32, credits: 320, icon: RefreshCw },
                  { name: 'Expand', count: 28, credits: 280, icon: TrendingUp },
                  { name: 'Quick Edit', count: 67, credits: 335, icon: Zap }
                ].map((op) => {
                  const Icon = op.icon;
                  return (
                    <Card key={op.name}>
                      <CardContent className="p-4">
                        <div className="flex items-center gap-2 mb-2">
                          <Icon className="w-4 h-4 text-blue-500" />
                          <span className="text-sm font-medium">{op.name}</span>
                        </div>
                        <div className="space-y-1">
                          <div className="text-lg font-bold">{op.count}</div>
                          <div className="text-xs text-gray-500">{op.credits} credits</div>
                        </div>
                      </CardContent>
                    </Card>
                  );
                })}
              </div>
            </TabsContent>
          </Tabs>
        </CardContent>
      </Card>
      
      {/* Settings Panel */}
      <AnimatePresence>
        {showSettings && (
          <motion.div
            initial={{ opacity: 0, height: 0 }}
            animate={{ opacity: 1, height: 'auto' }}
            exit={{ opacity: 0, height: 0 }}
          >
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Settings className="w-5 h-5" />
                  Credit Settings
                </CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div className="space-y-2">
                    <label className="text-sm font-medium">Low Credit Alert Threshold</label>
                    <div className="flex items-center gap-2">
                      <input
                        type="number"
                        value={credits.alertThreshold}
                        onChange={(e) => setLowCreditAlert(parseInt(e.target.value))}
                        className="flex-1 px-3 py-2 border rounded-md text-sm"
                        min="0"
                        max={credits.total}
                      />
                      <span className="text-sm text-gray-500">credits</span>
                    </div>
                  </div>
                  
                  <div className="space-y-2">
                    <label className="text-sm font-medium">Monthly Credit Limit</label>
                    <div className="flex items-center gap-2">
                      <input
                        type="number"
                        value={credits.monthlyLimit}
                        onChange={(e) => updateCreditLimit(parseInt(e.target.value))}
                        className="flex-1 px-3 py-2 border rounded-md text-sm"
                        min="0"
                      />
                      <span className="text-sm text-gray-500">credits</span>
                    </div>
                  </div>
                </div>
                
                <Separator />
                
                <div className="flex items-center justify-between">
                  <span className="text-sm font-medium">Auto-refresh credits</span>
                  <Button variant="outline" size="sm">
                    Configure
                  </Button>
                </div>
                
                <div className="flex items-center justify-between">
                  <span className="text-sm font-medium">Usage notifications</span>
                  <Button variant="outline" size="sm">
                    Configure
                  </Button>
                </div>
              </CardContent>
            </Card>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
};

export default AICreditManager;