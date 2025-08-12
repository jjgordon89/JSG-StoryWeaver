import React, { useState, useEffect } from 'react';
import { useAdvancedAIStore } from '../../stores/advancedAIStore';
import { useProjectStore } from '../../stores/projectStore';
import type { ImageGenerationRequest, GeneratedImage, ImageStyle, ImageResolution } from '../../types/advancedAI';
import ImageDetailModal from './ImageDetailModal';

interface ImageGeneratorProps {
  onImageGenerated: (image: GeneratedImage) => void;
}

const ImageGenerator: React.FC<ImageGeneratorProps> = ({ onImageGenerated }) => {
  // State
  const [prompt, setPrompt] = useState('');
  const [selectedStyle, setSelectedStyle] = useState<ImageStyle>('realistic');
  const [selectedResolution, setSelectedResolution] = useState<string>('1024x1024');
  const [aspectRatio, setAspectRatio] = useState('1:1');
  const [quality, setQuality] = useState('standard');
  const [enhancePrompt, setEnhancePrompt] = useState(true);
  const [useStoryContext, setUseStoryContext] = useState(true);
  const [generateVariations, setGenerateVariations] = useState(false);
  const [variationCount, setVariationCount] = useState(2);
  const [selectedImage, setSelectedImage] = useState<GeneratedImage | null>(null);
  const [viewMode, setViewMode] = useState<'grid' | 'list'>('grid');
  const [showAdvancedSettings, setShowAdvancedSettings] = useState(false);

  // Stores
  const advancedAIStore = useAdvancedAIStore();
  const projectStore = useProjectStore();

  // Computed values
  const isGenerating = advancedAIStore.isGeneratingImage;
  const canGenerate = advancedAIStore.canGenerate;
  const projectImages = advancedAIStore.projectImages;
  const availableStyles: ImageStyle[] = ['realistic', 'artistic', 'fantasy', 'sci-fi', 'cartoon', 'sketch'];
  const availableResolutions: string[] = ['512x512', '768x768', '1024x1024', '1024x1792', '1792x1024'];
  const aspectRatios = ['1:1', '4:3', '3:4', '16:9', '9:16'];
  const qualityOptions = ['standard', 'high', 'ultra'];

  const estimatedCredits = advancedAIStore.estimateImageCredits({
    resolution: selectedResolution,
    quality,
    enhancePrompt,
    generateVariations,
    variationCount
  });

  // Quick prompts
  const quickPrompts = [
    'Character portrait',
    'Scene setting',
    'Action sequence',
    'Emotional moment',
    'Fantasy landscape',
    'Sci-fi environment',
    'Historical setting',
    'Book cover design'
  ];

  // Event handlers
  const handleGenerate = async () => {
    if (!prompt.trim() || !canGenerate) return;

    const request: ImageGenerationRequest = {
      prompt: prompt.trim(),
      style: selectedStyle,
      resolution: selectedResolution,
      aspectRatio,
      quality,
      enhancePrompt,
      useStoryContext,
      generateVariations,
      variationCount: generateVariations ? variationCount : 1,
      projectId: projectStore.currentProject?.id
    };

    try {
      const images = await advancedAIStore.generateImage(request);
      images.forEach(image => onImageGenerated(image));
      setPrompt(''); // Clear prompt after successful generation
    } catch (error) {
      console.error('Image generation failed:', error);
    }
  };

  const handleQuickPrompt = (quickPrompt: string) => {
    setPrompt(quickPrompt);
  };

  const handleImageClick = (image: GeneratedImage) => {
    setSelectedImage(image);
  };

  const handleDownloadImage = (image: GeneratedImage) => {
    const link = document.createElement('a');
    link.href = image.url;
    link.download = `${image.prompt.slice(0, 30)}.png`;
    link.click();
  };

  const handleCopyImageUrl = (image: GeneratedImage) => {
    navigator.clipboard.writeText(image.url);
  };

  const handleDeleteImage = (imageId: string) => {
    advancedAIStore.deleteImage(imageId);
  };

  const handleRegenerateImage = (image: GeneratedImage) => {
    const request: ImageGenerationRequest = {
      prompt: image.prompt,
      style: image.style,
      resolution: image.resolution,
      aspectRatio: image.aspectRatio,
      quality: image.quality,
      enhancePrompt: image.enhancedPrompt !== undefined,
      useStoryContext: image.storyContext !== undefined,
      generateVariations: false,
      variationCount: 1,
      projectId: projectStore.currentProject?.id
    };

    advancedAIStore.generateImage(request);
  };

  return (
    <div className="image-generator">
      {/* Generation Form */}
      <div className="generation-form">
        <div className="prompt-section">
          <label htmlFor="image-prompt">Image Prompt:</label>
          <textarea
            id="image-prompt"
            value={prompt}
            onChange={(e) => setPrompt(e.target.value)}
            placeholder="Describe the image you want to generate..."
            rows={3}
            className="prompt-textarea"
          />
          
          {/* Quick Prompts */}
          <div className="quick-prompts">
            <span className="quick-prompts-label">Quick prompts:</span>
            <div className="quick-prompt-buttons">
              {quickPrompts.map((quickPrompt, index) => (
                <button
                  key={index}
                  className="quick-prompt-btn"
                  onClick={() => handleQuickPrompt(quickPrompt)}
                >
                  {quickPrompt}
                </button>
              ))}
            </div>
          </div>
        </div>

        {/* Basic Settings */}
        <div className="basic-settings">
          <div className="setting-row">
            <div className="setting-group">
              <label htmlFor="art-style">Art Style:</label>
              <select
                id="art-style"
                value={selectedStyle}
                onChange={(e) => setSelectedStyle(e.target.value as ImageStyle)}
              >
                {availableStyles.map((style) => (
                  <option key={style} value={style}>
                    {style.charAt(0).toUpperCase() + style.slice(1)}
                  </option>
                ))}
              </select>
            </div>

            <div className="setting-group">
              <label htmlFor="resolution">Resolution:</label>
              <select
                id="resolution"
                value={selectedResolution}
                onChange={(e) => setSelectedResolution(e.target.value)}
              >
                {availableResolutions.map((resolution) => (
                  <option key={resolution} value={resolution}>
                    {resolution}
                  </option>
                ))}
              </select>
            </div>
          </div>

          <div className="setting-row">
            <div className="setting-group">
              <label htmlFor="aspect-ratio">Aspect Ratio:</label>
              <select
                id="aspect-ratio"
                value={aspectRatio}
                onChange={(e) => setAspectRatio(e.target.value)}
              >
                {aspectRatios.map((ratio) => (
                  <option key={ratio} value={ratio}>
                    {ratio}
                  </option>
                ))}
              </select>
            </div>

            <div className="setting-group">
              <label htmlFor="quality">Quality:</label>
              <select
                id="quality"
                value={quality}
                onChange={(e) => setQuality(e.target.value)}
              >
                {qualityOptions.map((qual) => (
                  <option key={qual} value={qual}>
                    {qual.charAt(0).toUpperCase() + qual.slice(1)}
                  </option>
                ))}
              </select>
            </div>
          </div>

          <div className="setting-checkboxes">
            <label className="checkbox-label">
              <input
                type="checkbox"
                checked={enhancePrompt}
                onChange={(e) => setEnhancePrompt(e.target.checked)}
              />
              Enhance prompt with AI
            </label>
            
            <label className="checkbox-label">
              <input
                type="checkbox"
                checked={useStoryContext}
                onChange={(e) => setUseStoryContext(e.target.checked)}
              />
              Use story context
            </label>
          </div>
        </div>

        {/* Advanced Settings Toggle */}
        <button
          className="advanced-settings-toggle"
          onClick={() => setShowAdvancedSettings(!showAdvancedSettings)}
        >
          <i className={`fas fa-chevron-${showAdvancedSettings ? 'up' : 'down'}`}></i>
          Advanced Settings
        </button>

        {/* Advanced Settings */}
        {showAdvancedSettings && (
          <div className="advanced-settings">
            <div className="setting-group">
              <label className="checkbox-label">
                <input
                  type="checkbox"
                  checked={generateVariations}
                  onChange={(e) => setGenerateVariations(e.target.checked)}
                />
                Generate variations
              </label>
              
              {generateVariations && (
                <div className="variation-settings">
                  <label htmlFor="variation-count">Number of variations:</label>
                  <input
                    id="variation-count"
                    type="number"
                    min="2"
                    max="4"
                    value={variationCount}
                    onChange={(e) => setVariationCount(Number(e.target.value))}
                  />
                </div>
              )}
            </div>
          </div>
        )}

        {/* Generation Controls */}
        <div className="generation-controls">
          <div className="credit-estimate">
            Estimated cost: <strong>{estimatedCredits} credits</strong>
          </div>
          
          <button
            className="generate-btn"
            onClick={handleGenerate}
            disabled={!prompt.trim() || !canGenerate || isGenerating}
          >
            {isGenerating ? (
              <>
                <div className="loading-spinner"></div>
                Generating...
              </>
            ) : (
              <>
                <i className="fas fa-image"></i>
                Generate Image
              </>
            )}
          </button>
        </div>
      </div>

      {/* Generated Images */}
      {projectImages.length > 0 && (
        <div className="generated-images">
          <div className="images-header">
            <h3>Generated Images</h3>
            <div className="view-controls">
              <button
                className={`view-btn ${viewMode === 'grid' ? 'active' : ''}`}
                onClick={() => setViewMode('grid')}
              >
                <i className="fas fa-th"></i>
              </button>
              <button
                className={`view-btn ${viewMode === 'list' ? 'active' : ''}`}
                onClick={() => setViewMode('list')}
              >
                <i className="fas fa-list"></i>
              </button>
            </div>
          </div>

          <div className={`images-gallery ${viewMode}`}>
            {projectImages.map((image: GeneratedImage) => (
              <div key={image.id} className="image-item">
                <div className="image-container" onClick={() => handleImageClick(image)}>
                  <img src={image.url} alt={image.prompt} />
                  <div className="image-overlay">
                    <div className="image-actions">
                      <button
                        className="action-btn"
                        onClick={(e) => {
                          e.stopPropagation();
                          handleDownloadImage(image);
                        }}
                        title="Download"
                      >
                        <i className="fas fa-download"></i>
                      </button>
                      <button
                        className="action-btn"
                        onClick={(e) => {
                          e.stopPropagation();
                          handleCopyImageUrl(image);
                        }}
                        title="Copy URL"
                      >
                        <i className="fas fa-copy"></i>
                      </button>
                      <button
                        className="action-btn"
                        onClick={(e) => {
                          e.stopPropagation();
                          handleRegenerateImage(image);
                        }}
                        title="Regenerate"
                      >
                        <i className="fas fa-redo"></i>
                      </button>
                      <button
                        className="action-btn delete"
                        onClick={(e) => {
                          e.stopPropagation();
                          handleDeleteImage(image.id);
                        }}
                        title="Delete"
                      >
                        <i className="fas fa-trash"></i>
                      </button>
                    </div>
                  </div>
                </div>
                
                {viewMode === 'list' && (
                  <div className="image-info">
                    <div className="image-prompt">{image.prompt}</div>
                    <div className="image-meta">
                      {image.style} • {image.resolution} • {image.quality} • {image.creditsUsed} credits
                    </div>
                    {image.enhancedPrompt && (
                      <div className="enhanced-prompt">
                        <strong>Enhanced:</strong> {image.enhancedPrompt}
                      </div>
                    )}
                  </div>
                )}
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Image Detail Modal */}
      {selectedImage && (
        <ImageDetailModal
          image={selectedImage}
          onClose={() => setSelectedImage(null)}
          onDownload={() => handleDownloadImage(selectedImage)}
          onCopy={() => handleCopyImageUrl(selectedImage)}
          onRegenerate={() => handleRegenerateImage(selectedImage)}
          onDelete={() => {
            handleDeleteImage(selectedImage.id);
            setSelectedImage(null);
          }}
        />
      )}
    </div>
  );
};

export default ImageGenerator;