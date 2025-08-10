import React, { useState, useRef, useCallback } from 'react';
import { Button } from '../../../../ui/components/common';
import { Card, CardContent, CardHeader, CardTitle } from '../../../../ui/components/common';
import { Textarea } from '../../../../ui/components/common';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../../../../ui/components/common';
import { Upload, AlertCircle, CheckCircle, X, FileText, Users, MapPin, BookOpen, Lightbulb, Loader2 } from 'lucide-react';
import { useAdvancedAIStore } from '../../../../stores/advancedAIStore';
import type { SmartImportRequest, SmartImportResult, ExtractedCharacter, ExtractedLocation } from '../../../../types/advancedAI';

interface SmartImportDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onImport: (data: any[], type: 'characters' | 'locations' | 'plot_points' | 'themes') => Promise<void>;
  projectId: string;
}

interface ImportProgress {
  stage: 'uploading' | 'analyzing' | 'extracting' | 'complete';
  progress: number;
  message: string;
}

const SmartImportDialog: React.FC<SmartImportDialogProps> = ({
  isOpen,
  onClose,
  onImport,
  projectId
}) => {
  const [content, setContent] = useState('');
  const [contentType, setContentType] = useState('novel');
  const [isProcessing, setIsProcessing] = useState(false);
  const [progress, setProgress] = useState<ImportProgress>({ stage: 'uploading', progress: 0, message: '' });
  const [analysisResult, setAnalysisResult] = useState<SmartImportResult | null>(null);
  const [selectedCharacters, setSelectedCharacters] = useState<Set<number>>(new Set());
  const [selectedLocations, setSelectedLocations] = useState<Set<number>>(new Set());
  const [selectedPlotPoints, setSelectedPlotPoints] = useState<Set<number>>(new Set());
  const [selectedThemes, setSelectedThemes] = useState<Set<number>>(new Set());
  const [step, setStep] = useState<'upload' | 'analyze' | 'review' | 'complete'>('upload');
  const fileInputRef = useRef<HTMLInputElement>(null);
  const { smartImportContent } = useAdvancedAIStore();

  const MAX_WORDS = 120000;
  const MAX_CHARACTERS = 30;

  const countWords = (text: string): number => {
    return text.trim().split(/\s+/).filter(word => word.length > 0).length;
  };

  const handleFileUpload = useCallback((event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    const allowedTypes = ['.txt', '.doc', '.docx', '.rtf', '.odt'];
    const fileExtension = '.' + file.name.split('.').pop()?.toLowerCase();
    
    if (!allowedTypes.includes(fileExtension)) {
      alert('Please select a supported file type: .txt, .doc, .docx, .rtf, .odt');
      return;
    }

    const reader = new FileReader();
    reader.onload = (e) => {
      const text = e.target?.result as string;
      const wordCount = countWords(text);
      
      if (wordCount > MAX_WORDS) {
        alert(`File is too large (${wordCount.toLocaleString()} words). Maximum allowed is ${MAX_WORDS.toLocaleString()} words.`);
        return;
      }
      
      setContent(text);
    };
    reader.readAsText(file);
  }, []);

  const handleContentChange = useCallback((value: string) => {
    const wordCount = countWords(value);
    if (wordCount <= MAX_WORDS) {
      setContent(value);
    }
  }, []);

  const analyzeContent = async () => {
    if (!content.trim()) {
      alert('Please provide content to analyze');
      return;
    }

    setIsProcessing(true);
    setProgress({ stage: 'analyzing', progress: 10, message: 'Initializing AI analysis...' });

    try {
      setProgress({ stage: 'analyzing', progress: 30, message: 'Analyzing content structure...' });
      
      const request: SmartImportRequest = {
        project_id: projectId,
        content: content,
        content_type: contentType
      };

      setProgress({ stage: 'extracting', progress: 60, message: 'Extracting characters and story elements...' });
      
      const result = await smartImportContent(request);
      
      setProgress({ stage: 'extracting', progress: 90, message: 'Processing extracted elements...' });
      
      // Limit characters to 30 as per requirements
      if (result.extracted_elements.characters && result.extracted_elements.characters.length > MAX_CHARACTERS) {
        result.extracted_elements.characters = result.extracted_elements.characters
          .sort((a, b) => (b as any).confidence - (a as any).confidence)
          .slice(0, MAX_CHARACTERS);
      }
      
      setAnalysisResult(result);
      
      // Pre-select high-confidence items
      if (result.extracted_elements.characters) {
        const highConfidenceChars = new Set(
          result.extracted_elements.characters
            .map((_, index) => index)
            .filter(index => (result.extracted_elements.characters![index] as any).confidence > 0.7)
        );
        setSelectedCharacters(new Set(Array.from(highConfidenceChars) as number[]));
      }
      
      if (result.extracted_elements.locations) {
        const highConfidenceLocations = new Set(
          result.extracted_elements.locations
            .map((_, index) => index)
            .filter(index => (result.extracted_elements.locations![index] as any).confidence > 0.7)
        );
        setSelectedLocations(new Set(Array.from(highConfidenceLocations) as number[]));
      }
      
      setProgress({ stage: 'complete', progress: 100, message: 'Analysis complete!' });
      setStep('review');
      
    } catch (error) {
      console.error('Analysis failed:', error);
      alert(`Analysis failed: ${error instanceof Error ? error.message : 'Unknown error'}`);
    } finally {
      setIsProcessing(false);
    }
  };

  const handleImportSelected = async () => {
    if (!analysisResult) return;

    setIsProcessing(true);
    try {
      // Import selected characters
      if (selectedCharacters.size > 0 && analysisResult.extracted_elements.characters) {
        const charactersToImport = Array.from(selectedCharacters).map(index => {
          const char = analysisResult.extracted_elements.characters![index] as ExtractedCharacter;
          return {
            name: char.name,
            description: char.description,
            traits: {
              personality: char.traits.join(', '),
              relationships: char.relationships.join(', ')
            },
            visibility: 'public',
            series_shared: false
          };
        });
        await onImport(charactersToImport, 'characters');
      }

      // Import selected locations
      if (selectedLocations.size > 0 && analysisResult.extracted_elements.locations) {
        const locationsToImport = Array.from(selectedLocations).map(index => {
          const loc = analysisResult.extracted_elements.locations![index] as ExtractedLocation;
          return {
            name: loc.name,
            element_type: 'location',
            description: loc.description,
            significance: loc.significance,
            visibility: 'always',
            series_shared: false
          };
        });
        await onImport(locationsToImport, 'locations');
      }

      // Import selected plot points
      if (selectedPlotPoints.size > 0 && analysisResult.extracted_elements.plot_points) {
        const plotPointsToImport = Array.from(selectedPlotPoints).map(index => ({
          name: `Plot Point ${index + 1}`,
          element_type: 'plot_thread',
          description: analysisResult.extracted_elements.plot_points![index],
          significance: 'Key story element',
          visibility: 'always',
          series_shared: false
        }));
        await onImport(plotPointsToImport, 'plot_points');
      }

      // Import selected themes
      if (selectedThemes.size > 0 && analysisResult.extracted_elements.themes) {
        const themesToImport = Array.from(selectedThemes).map(index => ({
          name: analysisResult.extracted_elements.themes![index],
          element_type: 'theme',
          description: `Theme identified from imported content`,
          significance: 'Thematic element',
          visibility: 'always',
          series_shared: false
        }));
        await onImport(themesToImport, 'themes');
      }

      setStep('complete');
    } catch (error) {
      console.error('Import failed:', error);
      alert(`Import failed: ${error instanceof Error ? error.message : 'Unknown error'}`);
    } finally {
      setIsProcessing(false);
    }
  };

  const resetDialog = () => {
    setContent('');
    setContentType('novel');
    setAnalysisResult(null);
    setSelectedCharacters(new Set());
    setSelectedLocations(new Set());
    setSelectedPlotPoints(new Set());
    setSelectedThemes(new Set());
    setStep('upload');
    setProgress({ stage: 'uploading', progress: 0, message: '' });
    if (fileInputRef.current) {
      fileInputRef.current.value = '';
    }
  };

  const handleClose = () => {
    resetDialog();
    onClose();
  };

  const toggleSelection = (set: Set<number>, setSet: React.Dispatch<React.SetStateAction<Set<number>>>, index: number) => {
    const newSet = new Set(set);
    if (newSet.has(index)) {
      newSet.delete(index);
    } else {
      newSet.add(index);
    }
    setSet(newSet);
  };

  if (!isOpen) return null;

  const wordCount = countWords(content);
  const isContentValid = content.trim().length > 0 && wordCount <= MAX_WORDS;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg shadow-xl max-w-6xl w-full mx-4 max-h-[90vh] overflow-hidden">
        <div className="flex items-center justify-between p-6 border-b">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Lightbulb className="h-5 w-5 text-blue-500" />
            Smart Import - Novel Analysis
          </h2>
          <Button variant="ghost" onClick={handleClose}>
            <X className="h-4 w-4" />
          </Button>
        </div>

        <div className="p-6 overflow-y-auto max-h-[calc(90vh-120px)]">
          {step === 'upload' && (
            <div className="space-y-6">
              <div className="bg-blue-50 p-4 rounded-lg">
                <h3 className="font-medium text-blue-900 mb-2">Smart Import Features</h3>
                <ul className="text-blue-800 text-sm space-y-1">
                  <li>• Analyze up to {MAX_WORDS.toLocaleString()} words</li>
                  <li>• Extract up to {MAX_CHARACTERS} characters automatically</li>
                  <li>• Identify locations, plot points, and themes</li>
                  <li>• AI-powered analysis with confidence scoring</li>
                  <li>• No credits required for analysis</li>
                </ul>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Content Type
                </label>
                <Select value={contentType} onValueChange={setContentType}>
                  <SelectTrigger className="w-full">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="novel">Novel/Manuscript</SelectItem>
                    <SelectItem value="chapter">Chapter</SelectItem>
                    <SelectItem value="outline">Outline</SelectItem>
                    <SelectItem value="notes">Story Notes</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <div className="space-y-4">
                <div className="text-center">
                  <div className="border-2 border-dashed border-gray-300 rounded-lg p-8">
                    <Upload className="h-12 w-12 text-gray-400 mx-auto mb-4" />
                    <h3 className="text-lg font-medium text-gray-900 mb-2">
                      Upload File
                    </h3>
                    <p className="text-gray-600 mb-4">
                      Select a file to analyze (up to {MAX_WORDS.toLocaleString()} words)
                    </p>
                    <input
                      ref={fileInputRef}
                      type="file"
                      accept=".txt,.doc,.docx,.rtf,.odt"
                      onChange={handleFileUpload}
                      className="hidden"
                    />
                    <Button onClick={() => fileInputRef.current?.click()}>
                      <FileText className="h-4 w-4 mr-2" />
                      Choose File
                    </Button>
                  </div>
                </div>

                <div className="text-center text-gray-500">
                  <span>or</span>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Paste Content Directly
                  </label>
                  <Textarea
                    value={content}
                    onChange={(e) => handleContentChange(e.target.value)}
                    placeholder="Paste your novel, chapter, or story content here..."
                    rows={12}
                    className="font-mono text-sm"
                  />
                  <div className="flex justify-between items-center mt-2 text-sm">
                    <span className={`${wordCount > MAX_WORDS ? 'text-red-600' : 'text-gray-600'}`}>
                      {wordCount.toLocaleString()} / {MAX_WORDS.toLocaleString()} words
                    </span>
                    {wordCount > MAX_WORDS && (
                      <span className="text-red-600 flex items-center gap-1">
                        <AlertCircle className="h-4 w-4" />
                        Content too large
                      </span>
                    )}
                  </div>
                </div>
              </div>

              <div className="flex justify-end">
                <Button 
                  onClick={analyzeContent}
                  disabled={!isContentValid || isProcessing}
                  className="flex items-center gap-2"
                >
                  {isProcessing ? (
                    <Loader2 className="h-4 w-4 animate-spin" />
                  ) : (
                    <Lightbulb className="h-4 w-4" />
                  )}
                  Analyze Content
                </Button>
              </div>
            </div>
          )}

          {step === 'analyze' && (
            <div className="space-y-6">
              <div className="text-center">
                <Loader2 className="h-12 w-12 text-blue-500 animate-spin mx-auto mb-4" />
                <h3 className="text-lg font-medium text-gray-900 mb-2">
                  {progress.message}
                </h3>
                <div className="w-full bg-gray-200 rounded-full h-2 mb-4">
                  <div 
                    className="bg-blue-500 h-2 rounded-full transition-all duration-300"
                    style={{ width: `${progress.progress}%` }}
                  />
                </div>
                <p className="text-gray-600">
                  This may take a few moments for large documents...
                </p>
              </div>
            </div>
          )}

          {step === 'review' && analysisResult && (
            <div className="space-y-6">
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                <Card>
                  <CardHeader className="pb-2">
                    <CardTitle className="text-sm flex items-center gap-2">
                      <Users className="h-4 w-4" />
                      Characters
                    </CardTitle>
                  </CardHeader>
                  <CardContent>
                    <div className="text-2xl font-bold text-blue-600">
                      {analysisResult.extracted_elements.characters?.length || 0}
                    </div>
                    <div className="text-xs text-gray-500">
                      {selectedCharacters.size} selected
                    </div>
                  </CardContent>
                </Card>

                <Card>
                  <CardHeader className="pb-2">
                    <CardTitle className="text-sm flex items-center gap-2">
                      <MapPin className="h-4 w-4" />
                      Locations
                    </CardTitle>
                  </CardHeader>
                  <CardContent>
                    <div className="text-2xl font-bold text-green-600">
                      {analysisResult.extracted_elements.locations?.length || 0}
                    </div>
                    <div className="text-xs text-gray-500">
                      {selectedLocations.size} selected
                    </div>
                  </CardContent>
                </Card>

                <Card>
                  <CardHeader className="pb-2">
                    <CardTitle className="text-sm flex items-center gap-2">
                      <BookOpen className="h-4 w-4" />
                      Plot Points
                    </CardTitle>
                  </CardHeader>
                  <CardContent>
                    <div className="text-2xl font-bold text-purple-600">
                      {analysisResult.extracted_elements.plot_points?.length || 0}
                    </div>
                    <div className="text-xs text-gray-500">
                      {selectedPlotPoints.size} selected
                    </div>
                  </CardContent>
                </Card>

                <Card>
                  <CardHeader className="pb-2">
                    <CardTitle className="text-sm flex items-center gap-2">
                      <Lightbulb className="h-4 w-4" />
                      Themes
                    </CardTitle>
                  </CardHeader>
                  <CardContent>
                    <div className="text-2xl font-bold text-orange-600">
                      {analysisResult.extracted_elements.themes?.length || 0}
                    </div>
                    <div className="text-xs text-gray-500">
                      {selectedThemes.size} selected
                    </div>
                  </CardContent>
                </Card>
              </div>

              <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                {/* Characters */}
                {analysisResult.extracted_elements.characters && analysisResult.extracted_elements.characters.length > 0 && (
                  <Card>
                    <CardHeader>
                      <CardTitle className="flex items-center gap-2">
                        <Users className="h-5 w-5" />
                        Characters ({analysisResult.extracted_elements.characters.length})
                      </CardTitle>
                    </CardHeader>
                    <CardContent>
                      <div className="space-y-3 max-h-64 overflow-y-auto">
                        {analysisResult.extracted_elements.characters.map((character, index) => {
                          const char = character as ExtractedCharacter;
                          return (
                            <div key={index} className="flex items-start gap-3 p-3 border rounded-lg">
                              <input
                                type="checkbox"
                                checked={selectedCharacters.has(index)}
                                onChange={() => toggleSelection(selectedCharacters, setSelectedCharacters, index)}
                                className="mt-1"
                              />
                              <div className="flex-1 min-w-0">
                                <div className="font-medium text-sm">{char.name}</div>
                                <div className="text-xs text-gray-600 truncate">{char.description}</div>
                                {char.traits.length > 0 && (
                                  <div className="text-xs text-blue-600 mt-1">
                                    {char.traits.slice(0, 2).join(', ')}
                                    {char.traits.length > 2 && '...'}
                                  </div>
                                )}
                                <div className="text-xs text-gray-500 mt-1">
                                  Confidence: {Math.round(((char as any).confidence || 0.8) * 100)}%
                                </div>
                              </div>
                            </div>
                          );
                        })}
                      </div>
                    </CardContent>
                  </Card>
                )}

                {/* Locations */}
                {analysisResult.extracted_elements.locations && analysisResult.extracted_elements.locations.length > 0 && (
                  <Card>
                    <CardHeader>
                      <CardTitle className="flex items-center gap-2">
                        <MapPin className="h-5 w-5" />
                        Locations ({analysisResult.extracted_elements.locations.length})
                      </CardTitle>
                    </CardHeader>
                    <CardContent>
                      <div className="space-y-3 max-h-64 overflow-y-auto">
                        {analysisResult.extracted_elements.locations.map((location, index) => {
                          const loc = location as ExtractedLocation;
                          return (
                            <div key={index} className="flex items-start gap-3 p-3 border rounded-lg">
                              <input
                                type="checkbox"
                                checked={selectedLocations.has(index)}
                                onChange={() => toggleSelection(selectedLocations, setSelectedLocations, index)}
                                className="mt-1"
                              />
                              <div className="flex-1 min-w-0">
                                <div className="font-medium text-sm">{loc.name}</div>
                                <div className="text-xs text-gray-600 truncate">{loc.description}</div>
                                {loc.atmosphere && (
                                  <div className="text-xs text-green-600 mt-1">{loc.atmosphere}</div>
                                )}
                                <div className="text-xs text-gray-500 mt-1">
                                  Confidence: {Math.round(((loc as any).confidence || 0.8) * 100)}%
                                </div>
                              </div>
                            </div>
                          );
                        })}
                      </div>
                    </CardContent>
                  </Card>
                )}
              </div>

              {/* Plot Points and Themes */}
              <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                {analysisResult.extracted_elements.plot_points && analysisResult.extracted_elements.plot_points.length > 0 && (
                  <Card>
                    <CardHeader>
                      <CardTitle className="flex items-center gap-2">
                        <BookOpen className="h-5 w-5" />
                        Plot Points ({analysisResult.extracted_elements.plot_points.length})
                      </CardTitle>
                    </CardHeader>
                    <CardContent>
                      <div className="space-y-2 max-h-48 overflow-y-auto">
                        {analysisResult.extracted_elements.plot_points.map((point, index) => (
                          <div key={index} className="flex items-start gap-3 p-2 border rounded">
                            <input
                              type="checkbox"
                              checked={selectedPlotPoints.has(index)}
                              onChange={() => toggleSelection(selectedPlotPoints, setSelectedPlotPoints, index)}
                              className="mt-1"
                            />
                            <div className="text-sm">{point}</div>
                          </div>
                        ))}
                      </div>
                    </CardContent>
                  </Card>
                )}

                {analysisResult.extracted_elements.themes && analysisResult.extracted_elements.themes.length > 0 && (
                  <Card>
                    <CardHeader>
                      <CardTitle className="flex items-center gap-2">
                        <Lightbulb className="h-5 w-5" />
                        Themes ({analysisResult.extracted_elements.themes.length})
                      </CardTitle>
                    </CardHeader>
                    <CardContent>
                      <div className="space-y-2 max-h-48 overflow-y-auto">
                        {analysisResult.extracted_elements.themes.map((theme, index) => (
                          <div key={index} className="flex items-start gap-3 p-2 border rounded">
                            <input
                              type="checkbox"
                              checked={selectedThemes.has(index)}
                              onChange={() => toggleSelection(selectedThemes, setSelectedThemes, index)}
                              className="mt-1"
                            />
                            <div className="text-sm">{theme}</div>
                          </div>
                        ))}
                      </div>
                    </CardContent>
                  </Card>
                )}
              </div>

              <div className="flex justify-between">
                <Button variant="outline" onClick={() => setStep('upload')}>
                  Back
                </Button>
                <Button 
                  onClick={handleImportSelected}
                  disabled={isProcessing || (selectedCharacters.size === 0 && selectedLocations.size === 0 && selectedPlotPoints.size === 0 && selectedThemes.size === 0)}
                  className="flex items-center gap-2"
                >
                  {isProcessing ? (
                    <Loader2 className="h-4 w-4 animate-spin" />
                  ) : (
                    <CheckCircle className="h-4 w-4" />
                  )}
                  Import Selected Items
                </Button>
              </div>
            </div>
          )}

          {step === 'complete' && (
            <div className="text-center space-y-6">
              <div className="flex justify-center">
                <CheckCircle className="h-16 w-16 text-green-500" />
              </div>
              <div>
                <h3 className="text-lg font-medium text-gray-900 mb-2">
                  Smart Import Complete!
                </h3>
                <p className="text-gray-600">
                  Successfully imported selected story elements to your Story Bible.
                </p>
              </div>
              <Button onClick={handleClose}>
                Close
              </Button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default SmartImportDialog;