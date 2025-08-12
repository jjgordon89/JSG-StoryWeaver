import React, { useState } from 'react';
import { motion } from 'framer-motion';
import { 
  Settings, 
  Brain, 
  Sliders, 
  Save, 
  RotateCcw, 
  Eye, 
  EyeOff, 
  CheckCircle, 
  AlertCircle, 
  Info, 
  Zap, 
  Shield, 
  Clock,
  TestTube
} from 'lucide-react';
import { Button } from '../../ui/components/common';
import { Card, CardContent, CardHeader, CardTitle } from '../../ui/components/common';
import { Input, Textarea, Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../../ui/components/common';
import { Label } from '../ui/label';
import { Slider } from '../ui/slider';
import { Switch } from '../ui/switch';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '../ui/tabs';
import { Badge } from '../ui/badge';
import { Separator } from '../ui/separator';
import { Alert, AlertDescription } from '../ui/alert';
import { useAISettings } from '../../hooks/useAI';

interface AISettingsPanelProps {
  className?: string;
}

interface AIProvider {
  id: string;
  name: string;
  status: 'connected' | 'disconnected' | 'error';
  models: string[];
  defaultModel: string;
  apiKey?: string;
  baseUrl?: string;
  maxTokens: number;
  costPerToken: number;
  features: string[];
}

interface AIModel {
  id: string;
  name: string;
  provider: string;
  contextLength: number;
  costPer1k: number;
  capabilities: string[];
  recommended: boolean;
}

export const AISettingsPanel: React.FC<AISettingsPanelProps> = ({ className = '' }) => {
  const [showApiKeys, setShowApiKeys] = useState(false);
  const [testingConnection, setTestingConnection] = useState<string | null>(null);
  const [hasUnsavedChanges, setHasUnsavedChanges] = useState(false);
  
  const {
    settings,
    updateSettings
  } = useAISettings();
  
  // Mock data for demonstration
  const [availableProviders] = useState<AIProvider[]>([
    {
      id: 'openai',
      name: 'OpenAI',
      status: 'connected',
      models: ['gpt-4', 'gpt-4-turbo', 'gpt-3.5-turbo'],
      defaultModel: 'gpt-4',
      maxTokens: 128000,
      costPerToken: 0.00003,
      features: ['Chat', 'Completion', 'Function Calling', 'Vision']
    },
    {
      id: 'claude',
      name: 'Anthropic Claude',
      status: 'connected',
      models: ['claude-3-opus', 'claude-3-sonnet', 'claude-3-haiku'],
      defaultModel: 'claude-3-sonnet',
      maxTokens: 200000,
      costPerToken: 0.000015,
      features: ['Chat', 'Completion', 'Long Context', 'Code Analysis']
    },
    {
      id: 'gemini',
      name: 'Google Gemini',
      status: 'disconnected',
      models: ['gemini-pro', 'gemini-pro-vision'],
      defaultModel: 'gemini-pro',
      maxTokens: 32000,
      costPerToken: 0.0000005,
      features: ['Chat', 'Completion', 'Vision', 'Multimodal']
    }
  ]);
  
  const [availableModels] = useState<AIModel[]>([
    {
      id: 'gpt-4',
      name: 'GPT-4',
      provider: 'openai',
      contextLength: 128000,
      costPer1k: 0.03,
      capabilities: ['Advanced reasoning', 'Code generation', 'Creative writing'],
      recommended: true
    },
    {
      id: 'claude-3-sonnet',
      name: 'Claude 3 Sonnet',
      provider: 'claude',
      contextLength: 200000,
      costPer1k: 0.015,
      capabilities: ['Long context', 'Analysis', 'Code review'],
      recommended: true
    },
    {
      id: 'gemini-pro',
      name: 'Gemini Pro',
      provider: 'gemini',
      contextLength: 32000,
      costPer1k: 0.0005,
      capabilities: ['Fast responses', 'Cost effective', 'Multimodal'],
      recommended: false
    }
  ]);
  
  const handleTestConnection = async (providerId: string) => {
    setTestingConnection(providerId);
    try {
      // Mock test connection
      await new Promise(resolve => setTimeout(resolve, 1000));
      // Handle success
    } catch (error) {
      // Handle error
    } finally {
      setTestingConnection(null);
    }
  };
  
  const handleSaveSettings = async () => {
    try {
      // Mock save settings
      await new Promise(resolve => setTimeout(resolve, 500));
      setHasUnsavedChanges(false);
    } catch (error) {
      // Handle error
    }
  };
  
  const getProviderStatusIcon = (status: string) => {
    switch (status) {
      case 'connected':
        return <CheckCircle className="w-4 h-4 text-green-500" />;
      case 'error':
        return <AlertCircle className="w-4 h-4 text-red-500" />;
      default:
        return <AlertCircle className="w-4 h-4 text-gray-400" />;
    }
  };
  
  const getProviderStatusColor = (status: string) => {
    switch (status) {
      case 'connected':
        return 'green';
      case 'error':
        return 'red';
      default:
        return 'gray';
    }
  };
  
  return (
    <div className={`space-y-6 ${className}`}>
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <Settings className="w-6 h-6 text-blue-500" />
          <h2 className="text-xl font-semibold">AI Settings</h2>
        </div>
        
        <div className="flex items-center gap-2">
          {hasUnsavedChanges && (
            <Badge variant="outline" className="text-amber-600">
              Unsaved changes
            </Badge>
          )}
          
          <Button variant="outline" size="sm" onClick={() => setHasUnsavedChanges(true)}>
            <RotateCcw className="w-4 h-4 mr-1" />
            Reset
          </Button>
          
          <Button size="sm" onClick={handleSaveSettings} disabled={!hasUnsavedChanges}>
            <Save className="w-4 h-4 mr-1" />
            Save
          </Button>
        </div>
      </div>
      
      <Tabs defaultValue="providers" className="space-y-6">
        <TabsList className="grid w-full grid-cols-4">
          <TabsTrigger value="providers">Providers</TabsTrigger>
          <TabsTrigger value="models">Models</TabsTrigger>
          <TabsTrigger value="behavior">Behavior</TabsTrigger>
          <TabsTrigger value="advanced">Advanced</TabsTrigger>
        </TabsList>
        
        {/* Providers Tab */}
        <TabsContent value="providers" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Brain className="w-5 h-5" />
                AI Providers
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              {availableProviders.map((provider) => (
                <div key={provider.id} className="border rounded-lg p-4 space-y-4">
                  {/* Provider Header */}
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-3">
                      {getProviderStatusIcon(provider.status)}
                      <div>
                        <h3 className="font-medium">{provider.name}</h3>
                        <div className="flex items-center gap-2 mt-1">
                          <Badge 
                            variant="outline" 
                            className={`text-${getProviderStatusColor(provider.status)}-600`}
                          >
                            {provider.status}
                          </Badge>
                          <span className="text-xs text-gray-500">
                            {provider.models.length} models available
                          </span>
                        </div>
                      </div>
                    </div>
                    
                    <div className="flex items-center gap-2">
                      <Button 
                        variant="outline" 
                        size="sm" 
                        onClick={() => handleTestConnection(provider.id)}
                        disabled={testingConnection === provider.id}
                      >
                        {testingConnection === provider.id ? (
                          <motion.div
                            animate={{ rotate: 360 }}
                            transition={{ duration: 1, repeat: Infinity, ease: "linear" }}
                          >
                            <TestTube className="w-4 h-4" />
                          </motion.div>
                        ) : (
                          <TestTube className="w-4 h-4" />
                        )}
                        Test
                      </Button>
                      
                      <Switch 
                        checked={provider.status === 'connected'}
                        onCheckedChange={(_checked) => {
                          // Handle provider enable/disable
                          setHasUnsavedChanges(true);
                        }}
                      />
                    </div>
                  </div>
                  
                  {/* Provider Configuration */}
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div className="space-y-2">
                      <Label htmlFor={`${provider.id}-api-key`}>API Key</Label>
                      <div className="flex items-center gap-2">
                        <Input
                          id={`${provider.id}-api-key`}
                          type={showApiKeys ? 'text' : 'password'}
                          placeholder="Enter API key"
                          value={provider.apiKey || ''}
                          onChange={(_e) => {
                            // Handle API key change
                            setHasUnsavedChanges(true);
                          }}
                        />
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => setShowApiKeys(!showApiKeys)}
                        >
                          {showApiKeys ? <EyeOff className="w-4 h-4" /> : <Eye className="w-4 h-4" />}
                        </Button>
                      </div>
                    </div>
                    
                    <div className="space-y-2">
                      <Label htmlFor={`${provider.id}-model`}>Default Model</Label>
                      <Select value={provider.defaultModel}>
                        <SelectTrigger>
                          <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                          {provider.models.map((model) => (
                            <SelectItem key={model} value={model}>
                              {model}
                            </SelectItem>
                          ))}
                        </SelectContent>
                      </Select>
                    </div>
                    
                    {provider.baseUrl && (
                      <div className="space-y-2 md:col-span-2">
                        <Label htmlFor={`${provider.id}-base-url`}>Base URL</Label>
                        <Input
                          id={`${provider.id}-base-url`}
                          placeholder="https://api.example.com/v1"
                          value={provider.baseUrl}
                          onChange={(_e) => {
                            // Handle base URL change
                            setHasUnsavedChanges(true);
                          }}
                        />
                      </div>
                    )}
                  </div>
                  
                  {/* Provider Features */}
                  <div className="space-y-2">
                    <Label>Features</Label>
                    <div className="flex flex-wrap gap-2">
                      {provider.features.map((feature) => (
                        <Badge key={feature} variant="secondary">
                          {feature}
                        </Badge>
                      ))}
                    </div>
                  </div>
                  
                  {/* Provider Stats */}
                  <div className="grid grid-cols-2 md:grid-cols-4 gap-4 pt-2 border-t">
                    <div className="text-center">
                      <div className="text-sm text-gray-500">Max Tokens</div>
                      <div className="font-medium">{provider.maxTokens.toLocaleString()}</div>
                    </div>
                    <div className="text-center">
                      <div className="text-sm text-gray-500">Cost/Token</div>
                      <div className="font-medium">${provider.costPerToken.toFixed(6)}</div>
                    </div>
                    <div className="text-center">
                      <div className="text-sm text-gray-500">Models</div>
                      <div className="font-medium">{provider.models.length}</div>
                    </div>
                    <div className="text-center">
                      <div className="text-sm text-gray-500">Status</div>
                      <div className="font-medium capitalize">{provider.status}</div>
                    </div>
                  </div>
                </div>
              ))}
            </CardContent>
          </Card>
        </TabsContent>
        
        {/* Models Tab */}
        <TabsContent value="models" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Zap className="w-5 h-5" />
                Model Configuration
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="grid gap-4">
                {availableModels.map((model) => (
                  <div key={model.id} className="border rounded-lg p-4 space-y-3">
                    <div className="flex items-center justify-between">
                      <div className="flex items-center gap-3">
                        <div>
                          <div className="flex items-center gap-2">
                            <h3 className="font-medium">{model.name}</h3>
                            {model.recommended && (
                              <Badge variant="default" className="text-xs">
                                Recommended
                              </Badge>
                            )}
                          </div>
                          <div className="text-sm text-gray-500 capitalize">
                            {model.provider} • {model.contextLength.toLocaleString()} tokens
                          </div>
                        </div>
                      </div>
                      
                      <div className="text-right">
                        <div className="font-medium">${model.costPer1k}/1K tokens</div>
                        <div className="text-sm text-gray-500">Cost</div>
                      </div>
                    </div>
                    
                    <div className="space-y-2">
                      <Label>Capabilities</Label>
                      <div className="flex flex-wrap gap-2">
                        {model.capabilities.map((capability) => (
                          <Badge key={capability} variant="outline">
                            {capability}
                          </Badge>
                        ))}
                      </div>
                    </div>
                    
                    <div className="grid grid-cols-3 gap-4 pt-2 border-t text-center">
                      <div>
                        <div className="text-sm text-gray-500">Context</div>
                        <div className="font-medium">{model.contextLength.toLocaleString()}</div>
                      </div>
                      <div>
                        <div className="text-sm text-gray-500">Provider</div>
                        <div className="font-medium capitalize">{model.provider}</div>
                      </div>
                      <div>
                        <div className="text-sm text-gray-500">Cost/1K</div>
                        <div className="font-medium">${model.costPer1k}</div>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </CardContent>
          </Card>
        </TabsContent>
        
        {/* Behavior Tab */}
        <TabsContent value="behavior" className="space-y-4">
          <div className="grid gap-4">
            {/* Writing Preferences */}
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Sliders className="w-5 h-5" />
                  Writing Preferences
                </CardTitle>
              </CardHeader>
              <CardContent className="space-y-6">
                <div className="space-y-4">
                  <div className="space-y-2">
                    <Label>Creativity Level</Label>
                    <Slider
                      value={[settings?.creativity || 0.7]}
                      onValueChange={([value]) => {
                        updateSettings.global({ creativity: value });
                        setHasUnsavedChanges(true);
                      }}
                      max={1}
                      min={0}
                      step={0.1}
                      className="w-full"
                    />
                    <div className="flex justify-between text-xs text-gray-500">
                      <span>Conservative</span>
                      <span>Balanced</span>
                      <span>Creative</span>
                    </div>
                  </div>
                  
                  <div className="space-y-2">
                    <Label>Response Length</Label>
                    <Select value={settings?.responseLength || 'medium'}>
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="short">Short (50-150 words)</SelectItem>
                        <SelectItem value="medium">Medium (150-300 words)</SelectItem>
                        <SelectItem value="long">Long (300-500 words)</SelectItem>
                        <SelectItem value="very-long">Very Long (500+ words)</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                  
                  <div className="space-y-2">
                    <Label>Writing Style</Label>
                    <Select value={settings?.writingStyle || 'balanced'}>
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="formal">Formal</SelectItem>
                        <SelectItem value="casual">Casual</SelectItem>
                        <SelectItem value="balanced">Balanced</SelectItem>
                        <SelectItem value="creative">Creative</SelectItem>
                        <SelectItem value="technical">Technical</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                  
                  <div className="space-y-2">
                    <Label>Custom Instructions</Label>
                    <Textarea
                      placeholder="Add any custom instructions for AI behavior..."
                      value={settings?.customInstructions || ''}
                      onChange={(e) => {
                        updateSettings.global({ customInstructions: e.target.value });
                        setHasUnsavedChanges(true);
                      }}
                      rows={3}
                    />
                  </div>
                </div>
              </CardContent>
            </Card>
            
            {/* Feature Toggles */}
            <Card>
              <CardHeader>
                <CardTitle>Feature Settings</CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="flex items-center justify-between">
                  <div className="space-y-1">
                    <Label>Auto-save generated content</Label>
                    <p className="text-sm text-gray-500">Automatically save AI responses as cards</p>
                  </div>
                  <Switch 
                    checked={settings?.autoSave || false}
                    onCheckedChange={(checked) => {
                      updateSettings.global({ autoSave: checked });
                      setHasUnsavedChanges(true);
                    }}
                  />
                </div>
                
                <Separator />
                
                <div className="flex items-center justify-between">
                  <div className="space-y-1">
                    <Label>Show cost estimates</Label>
                    <p className="text-sm text-gray-500">Display estimated costs before AI operations</p>
                  </div>
                  <Switch 
                    checked={settings?.showCosts || true}
                    onCheckedChange={(checked) => {
                      updateSettings.global({ showCosts: checked });
                      setHasUnsavedChanges(true);
                    }}
                  />
                </div>
                
                <Separator />
                
                <div className="flex items-center justify-between">
                  <div className="space-y-1">
                    <Label>Enable streaming responses</Label>
                    <p className="text-sm text-gray-500">Stream AI responses in real-time</p>
                  </div>
                  <Switch 
                    checked={settings?.enableStreaming || true}
                    onCheckedChange={(checked) => {
                      updateSettings.global({ enableStreaming: checked });
                      setHasUnsavedChanges(true);
                    }}
                  />
                </div>
                
                <Separator />
                
                <div className="flex items-center justify-between">
                  <div className="space-y-1">
                    <Label>Context awareness</Label>
                    <p className="text-sm text-gray-500">Include document context in AI requests</p>
                  </div>
                  <Switch 
                    checked={settings?.contextAware || true}
                    onCheckedChange={(checked) => {
                      updateSettings.global({ contextAware: checked });
                      setHasUnsavedChanges(true);
                    }}
                  />
                </div>
              </CardContent>
            </Card>
          </div>
        </TabsContent>
        
        {/* Advanced Tab */}
        <TabsContent value="advanced" className="space-y-4">
          <div className="grid gap-4">
            {/* Performance Settings */}
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Clock className="w-5 h-5" />
                  Performance & Limits
                </CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div className="space-y-2">
                    <Label>Request Timeout (seconds)</Label>
                    <Input
                      type="number"
                      value={settings?.requestTimeout || 30}
                      onChange={(e) => {
                        updateSettings.global({ requestTimeout: parseInt(e.target.value) });
                        setHasUnsavedChanges(true);
                      }}
                      min="5"
                      max="300"
                    />
                  </div>
                  
                  <div className="space-y-2">
                    <Label>Max Concurrent Requests</Label>
                    <Input
                      type="number"
                      value={settings?.maxConcurrentRequests || 3}
                      onChange={(e) => {
                        updateSettings.global({ maxConcurrentRequests: parseInt(e.target.value) });
                        setHasUnsavedChanges(true);
                      }}
                      min="1"
                      max="10"
                    />
                  </div>
                  
                  <div className="space-y-2">
                    <Label>Retry Attempts</Label>
                    <Input
                      type="number"
                      value={settings?.retryAttempts || 3}
                      onChange={(e) => {
                        updateSettings.global({ retryAttempts: parseInt(e.target.value) });
                        setHasUnsavedChanges(true);
                      }}
                      min="0"
                      max="5"
                    />
                  </div>
                  
                  <div className="space-y-2">
                    <Label>Cache Duration (minutes)</Label>
                    <Input
                      type="number"
                      value={settings?.cacheDuration || 60}
                      onChange={(e) => {
                        updateSettings.global({ cacheDuration: parseInt(e.target.value) });
                        setHasUnsavedChanges(true);
                      }}
                      min="0"
                      max="1440"
                    />
                  </div>
                </div>
              </CardContent>
            </Card>
            
            {/* Security Settings */}
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Shield className="w-5 h-5" />
                  Security & Privacy
                </CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <Alert>
                  <Info className="w-4 h-4" />
                  <AlertDescription>
                    API keys are stored securely and encrypted. They are never shared or logged.
                  </AlertDescription>
                </Alert>
                
                <div className="space-y-4">
                  <div className="flex items-center justify-between">
                    <div className="space-y-1">
                      <Label>Log AI requests</Label>
                      <p className="text-sm text-gray-500">Keep logs for debugging and analytics</p>
                    </div>
                    <Switch 
                      checked={settings?.logRequests || false}
                      onCheckedChange={(checked) => {
                        updateSettings.global({ logRequests: checked });
                        setHasUnsavedChanges(true);
                      }}
                    />
                  </div>
                  
                  <Separator />
                  
                  <div className="flex items-center justify-between">
                    <div className="space-y-1">
                      <Label>Share usage analytics</Label>
                      <p className="text-sm text-gray-500">Help improve the service with anonymous usage data</p>
                    </div>
                    <Switch 
                      checked={settings?.shareAnalytics || false}
                      onCheckedChange={(checked) => {
                        updateSettings.global({ shareAnalytics: checked });
                        setHasUnsavedChanges(true);
                      }}
                    />
                  </div>
                  
                  <Separator />
                  
                  <div className="space-y-2">
                    <Label>Data Retention (days)</Label>
                    <Select value={settings?.dataRetention?.toString() || '30'} onValueChange={(value) => updateSettings.global({ dataRetention: parseInt(value) })}>
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="7">7 days</SelectItem>
                        <SelectItem value="30">30 days</SelectItem>
                        <SelectItem value="90">90 days</SelectItem>
                        <SelectItem value="365">1 year</SelectItem>
                        <SelectItem value="0">Never delete</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                </div>
              </CardContent>
            </Card>
            
            {/* Debug Settings */}
            <Card>
              <CardHeader>
                <CardTitle>Debug & Development</CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="flex items-center justify-between">
                  <div className="space-y-1">
                    <Label>Enable debug mode</Label>
                    <p className="text-sm text-gray-500">Show detailed logs and debug information</p>
                  </div>
                  <Switch 
                    checked={settings?.debugMode || false}
                    onCheckedChange={(checked) => {
                      updateSettings.global({ debugMode: checked });
                      setHasUnsavedChanges(true);
                    }}
                  />
                </div>
                
                <Separator />
                
                <div className="flex items-center justify-between">
                  <div className="space-y-1">
                    <Label>Mock AI responses</Label>
                    <p className="text-sm text-gray-500">Use mock responses for testing (no API calls)</p>
                  </div>
                  <Switch 
                    checked={settings?.mockMode || false}
                    onCheckedChange={(checked) => {
                      updateSettings.global({ mockMode: checked });
                      setHasUnsavedChanges(true);
                    }}
                  />
                </div>
              </CardContent>
            </Card>
          </div>
        </TabsContent>
      </Tabs>
    </div>
  );
};

export default AISettingsPanel;