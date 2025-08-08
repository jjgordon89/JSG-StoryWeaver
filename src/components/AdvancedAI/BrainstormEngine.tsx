import React, { useState, useEffect } from 'react';
import { useAdvancedAIStore } from '../../stores/advancedAIStore';
import { useProjectStore } from '../../stores/projectStore';
import type { BrainstormSession, BrainstormIdea, BrainstormRequest } from '../../types/advancedAI';
import IdeaDetailModal from './IdeaDetailModal';

interface BrainstormEngineProps {
  currentSession: BrainstormSession | null;
  onSessionCreated: (session: BrainstormSession) => void;
}

const BrainstormEngine: React.FC<BrainstormEngineProps> = ({
  currentSession,
  onSessionCreated
}) => {
  // State
  const [sessionName, setSessionName] = useState('');
  const [sessionTopic, setSessionTopic] = useState('');
  const [creativityLevel, setCreativityLevel] = useState(0.7);
  const [useStoryContext, setUseStoryContext] = useState(true);
  const [ideasPerGeneration, setIdeasPerGeneration] = useState(5);
  const [filterCategory, setFilterCategory] = useState('all');
  const [sortBy, setSortBy] = useState('created');
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedIdea, setSelectedIdea] = useState<BrainstormIdea | null>(null);
  const [showOnlyKeepers, setShowOnlyKeepers] = useState(false);

  // Stores
  const advancedAIStore = useAdvancedAIStore();
  const projectStore = useProjectStore();

  // Computed values
  const isGenerating = advancedAIStore.isGenerating;
  const canGenerate = advancedAIStore.canGenerate;
  const brainstormSessions = advancedAIStore.brainstormSessions;
  const estimatedCredits = advancedAIStore.estimateBrainstormCredits({
    ideasPerGeneration,
    creativityLevel,
    useStoryContext
  });

  const categories = ['all', 'plot', 'character', 'setting', 'theme', 'conflict', 'dialogue', 'other'];
  const sortOptions = [
    { value: 'created', label: 'Date Created' },
    { value: 'rating', label: 'Rating' },
    { value: 'category', label: 'Category' },
    { value: 'alphabetical', label: 'Alphabetical' }
  ];

  // Filter and sort ideas
  const filteredIdeas = currentSession?.ideas.filter(idea => {
    const matchesCategory = filterCategory === 'all' || idea.category === filterCategory;
    const matchesSearch = !searchQuery || 
      idea.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      idea.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
      idea.tags.some(tag => tag.toLowerCase().includes(searchQuery.toLowerCase()));
    const matchesKeepers = !showOnlyKeepers || idea.isKeeper;
    
    return matchesCategory && matchesSearch && matchesKeepers;
  }).sort((a, b) => {
    switch (sortBy) {
      case 'rating':
        return (b.rating || 0) - (a.rating || 0);
      case 'category':
        return a.category.localeCompare(b.category);
      case 'alphabetical':
        return a.title.localeCompare(b.title);
      default:
        return new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime();
    }
  }) || [];

  const keeperCount = currentSession?.ideas.filter(idea => idea.isKeeper).length || 0;
  const totalIdeas = currentSession?.ideas.length || 0;

  // Event handlers
  const handleCreateSession = async () => {
    if (!sessionName.trim() || !sessionTopic.trim()) return;

    const request: BrainstormRequest = {
      sessionName: sessionName.trim(),
      topic: sessionTopic.trim(),
      creativityLevel,
      useStoryContext,
      ideasPerGeneration,
      projectId: projectStore.currentProject?.id
    };

    try {
      const session = await advancedAIStore.createBrainstormSession(request);
      onSessionCreated(session);
      setSessionName('');
      setSessionTopic('');
    } catch (error) {
      console.error('Failed to create brainstorm session:', error);
    }
  };

  const handleGenerateIdeas = async () => {
    if (!currentSession || !canGenerate) return;

    const request: BrainstormRequest = {
      sessionName: currentSession.name,
      topic: currentSession.topic,
      creativityLevel,
      useStoryContext,
      ideasPerGeneration,
      projectId: projectStore.currentProject?.id
    };

    try {
      await advancedAIStore.generateIdeas(currentSession.id, request);
    } catch (error) {
      console.error('Failed to generate ideas:', error);
    }
  };

  const handleIdeaClick = (idea: BrainstormIdea) => {
    setSelectedIdea(idea);
  };

  const handleToggleKeeper = (ideaId: string) => {
    advancedAIStore.toggleIdeaKeeper(ideaId);
  };

  const handleRateIdea = (ideaId: string, rating: number) => {
    advancedAIStore.rateIdea(ideaId, rating);
  };

  const handleDeleteIdea = (ideaId: string) => {
    advancedAIStore.deleteIdea(ideaId);
  };

  const handleExportKeepers = () => {
    if (!currentSession) return;
    
    const keepers = currentSession.ideas.filter(idea => idea.isKeeper);
    advancedAIStore.exportKeepersToStoryBible(keepers);
  };

  const handleDuplicateIdea = (idea: BrainstormIdea) => {
    advancedAIStore.duplicateIdea(idea);
  };

  const getRatingStars = (rating: number) => {
    return Array.from({ length: 5 }, (_, i) => (
      <i 
        key={i} 
        className={`fas fa-star ${i < rating ? 'filled' : 'empty'}`}
      ></i>
    ));
  };

  return (
    <div className="brainstorm-engine">
      {/* Session Creation/Selection */}
      {!currentSession ? (
        <div className="session-creation">
          <h3>Create New Brainstorm Session</h3>
          
          <div className="creation-form">
            <div className="form-group">
              <label htmlFor="session-name">Session Name:</label>
              <input
                id="session-name"
                type="text"
                value={sessionName}
                onChange={(e) => setSessionName(e.target.value)}
                placeholder="Enter session name..."
              />
            </div>

            <div className="form-group">
              <label htmlFor="session-topic">Topic/Focus:</label>
              <textarea
                id="session-topic"
                value={sessionTopic}
                onChange={(e) => setSessionTopic(e.target.value)}
                placeholder="What do you want to brainstorm about?"
                rows={3}
              />
            </div>

            <div className="form-row">
              <div className="form-group">
                <label htmlFor="creativity-level">Creativity Level:</label>
                <input
                  id="creativity-level"
                  type="range"
                  min="0.1"
                  max="1.0"
                  step="0.1"
                  value={creativityLevel}
                  onChange={(e) => setCreativityLevel(Number(e.target.value))}
                />
                <span className="range-value">{Math.round(creativityLevel * 100)}%</span>
              </div>

              <div className="form-group">
                <label htmlFor="ideas-count">Ideas per Generation:</label>
                <input
                  id="ideas-count"
                  type="number"
                  min="3"
                  max="10"
                  value={ideasPerGeneration}
                  onChange={(e) => setIdeasPerGeneration(Number(e.target.value))}
                />
              </div>
            </div>

            <div className="form-group">
              <label className="checkbox-label">
                <input
                  type="checkbox"
                  checked={useStoryContext}
                  onChange={(e) => setUseStoryContext(e.target.checked)}
                />
                Use story context for relevant ideas
              </label>
            </div>

            <div className="creation-controls">
              <div className="credit-estimate">
                Estimated cost: <strong>{estimatedCredits} credits</strong>
              </div>
              
              <button
                className="create-session-btn"
                onClick={handleCreateSession}
                disabled={!sessionName.trim() || !sessionTopic.trim() || isGenerating}
              >
                <i className="fas fa-lightbulb"></i>
                Create Session & Generate Ideas
              </button>
            </div>
          </div>

          {/* Previous Sessions */}
          {brainstormSessions.length > 0 && (
            <div className="previous-sessions">
              <h4>Previous Sessions</h4>
              <div className="sessions-list">
                {brainstormSessions.map((session) => (
                  <div key={session.id} className="session-item">
                    <div className="session-info">
                      <h5>{session.name}</h5>
                      <p>{session.topic}</p>
                      <span className="session-meta">
                        {session.ideas.length} ideas • {session.ideas.filter(i => i.isKeeper).length} keepers
                      </span>
                    </div>
                    <button
                      className="load-session-btn"
                      onClick={() => advancedAIStore.loadBrainstormSession(session.id)}
                    >
                      Load
                    </button>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      ) : (
        <div className="active-session">
          {/* Session Header */}
          <div className="session-header">
            <div className="session-info">
              <h3>{currentSession.name}</h3>
              <p>{currentSession.topic}</p>
              <div className="session-stats">
                <span>{totalIdeas} total ideas</span>
                <span>{keeperCount} keepers</span>
              </div>
            </div>
            
            <div className="session-actions">
              <button
                className="generate-more-btn"
                onClick={handleGenerateIdeas}
                disabled={!canGenerate || isGenerating}
              >
                {isGenerating ? (
                  <>
                    <div className="loading-spinner"></div>
                    Generating...
                  </>
                ) : (
                  <>
                    <i className="fas fa-plus"></i>
                    Generate More Ideas
                  </>
                )}
              </button>
              
              {keeperCount > 0 && (
                <button className="export-btn" onClick={handleExportKeepers}>
                  <i className="fas fa-download"></i>
                  Export Keepers
                </button>
              )}
              
              <button 
                className="new-session-btn"
                onClick={() => advancedAIStore.clearCurrentSession()}
              >
                <i className="fas fa-plus-circle"></i>
                New Session
              </button>
            </div>
          </div>

          {/* Filters and Controls */}
          <div className="ideas-controls">
            <div className="filters">
              <div className="filter-group">
                <label htmlFor="category-filter">Category:</label>
                <select
                  id="category-filter"
                  value={filterCategory}
                  onChange={(e) => setFilterCategory(e.target.value)}
                >
                  {categories.map((category) => (
                    <option key={category} value={category}>
                      {category.charAt(0).toUpperCase() + category.slice(1)}
                    </option>
                  ))}
                </select>
              </div>

              <div className="filter-group">
                <label htmlFor="sort-by">Sort by:</label>
                <select
                  id="sort-by"
                  value={sortBy}
                  onChange={(e) => setSortBy(e.target.value)}
                >
                  {sortOptions.map((option) => (
                    <option key={option.value} value={option.value}>
                      {option.label}
                    </option>
                  ))}
                </select>
              </div>

              <div className="filter-group">
                <input
                  type="text"
                  placeholder="Search ideas..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="search-input"
                />
              </div>

              <label className="checkbox-label">
                <input
                  type="checkbox"
                  checked={showOnlyKeepers}
                  onChange={(e) => setShowOnlyKeepers(e.target.checked)}
                />
                Show only keepers
              </label>
            </div>
          </div>

          {/* Ideas List */}
          <div className="ideas-list">
            {filteredIdeas.length === 0 ? (
              <div className="no-ideas">
                <p>No ideas match your current filters.</p>
              </div>
            ) : (
              filteredIdeas.map((idea) => (
                <div key={idea.id} className="idea-item">
                  <div className="idea-header">
                    <div className="idea-title-section">
                      <h4 className="idea-title" onClick={() => handleIdeaClick(idea)}>
                        {idea.title}
                      </h4>
                      <span className="idea-category">{idea.category}</span>
                    </div>
                    
                    <div className="idea-actions">
                      <button
                        className={`keeper-btn ${idea.isKeeper ? 'active' : ''}`}
                        onClick={() => handleToggleKeeper(idea.id)}
                        title={idea.isKeeper ? 'Remove from keepers' : 'Mark as keeper'}
                      >
                        <i className={`fas ${idea.isKeeper ? 'fa-star' : 'fa-star-o'}`}></i>
                      </button>
                      
                      <div className="rating">
                        {Array.from({ length: 5 }, (_, i) => (
                          <button
                            key={i}
                            className={`star-btn ${i < (idea.rating || 0) ? 'filled' : 'empty'}`}
                            onClick={() => handleRateIdea(idea.id, i + 1)}
                          >
                            <i className="fas fa-star"></i>
                          </button>
                        ))}
                      </div>
                      
                      <button
                        className="duplicate-btn"
                        onClick={() => handleDuplicateIdea(idea)}
                        title="Duplicate idea"
                      >
                        <i className="fas fa-copy"></i>
                      </button>
                      
                      <button
                        className="delete-btn"
                        onClick={() => handleDeleteIdea(idea.id)}
                        title="Delete idea"
                      >
                        <i className="fas fa-trash"></i>
                      </button>
                    </div>
                  </div>
                  
                  <p className="idea-description">{idea.description}</p>
                  
                  {idea.tags.length > 0 && (
                    <div className="idea-tags">
                      {idea.tags.map((tag, index) => (
                        <span key={index} className="tag">{tag}</span>
                      ))}
                    </div>
                  )}
                  
                  {idea.notes && (
                    <div className="idea-notes">
                      <strong>Notes:</strong> {idea.notes}
                    </div>
                  )}
                </div>
              ))
            )}
          </div>
        </div>
      )}

      {/* Idea Detail Modal */}
      {selectedIdea && (
        <IdeaDetailModal
          idea={selectedIdea}
          onClose={() => setSelectedIdea(null)}
          onUpdate={(updatedIdea) => {
            advancedAIStore.updateIdea(updatedIdea);
            setSelectedIdea(updatedIdea);
          }}
          onDelete={() => {
            handleDeleteIdea(selectedIdea.id);
            setSelectedIdea(null);
          }}
          onDuplicate={() => handleDuplicateIdea(selectedIdea)}
        />
      )}
    </div>
  );
};

export default BrainstormEngine;