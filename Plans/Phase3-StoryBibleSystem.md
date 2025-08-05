# Phase 3: Story Bible System (Weeks 11-15)

## Overview
Implement the comprehensive Story Bible system that serves as the centralized knowledge base for all story elements, including characters, worldbuilding, outlines, and scenes with advanced visibility controls and AI integration.

## Key Objectives
- Story Bible foundation (Braindump, Synopsis, Genre, Style)
- Characters and Worldbuilding with visibility controls
- Outline system with document linking and Acts/dividers
- Scenes & Draft functionality with validation
- Series-level sharing of Story Bible elements
- POV and Tense settings (global and per-chapter)
- Story Bible detection and highlighting in text

## Technical Tasks

### Week 11: Story Bible Foundation
- [ ] Create Story Bible core structure and database schema
- [ ] Implement Braindump free-form text area with AI influence
- [ ] Build Synopsis system with AI generation capabilities
- [ ] Add Genre and Style selection with examples
- [ ] Create Style Examples system (up to 1,000 words)
- [ ] Implement POV and Tense settings (global and per-chapter)
- [ ] Add Story Bible boxes for organized text fields
- [ ] Build AI generation for all Story Bible elements

### Week 12: Characters System
- [ ] Create character profiles with customizable traits
- [ ] Implement character trait visibility controls
- [ ] Build character relationship mapping
- [ ] Add character import from text/files (60K words, 30 chars max)
- [ ] Create CSV import for unlimited characters
- [ ] Implement character templates and archetypes
- [ ] Add character consistency tracking
- [ ] Build character POV assignment system

### Week 13: Worldbuilding System
- [ ] Create worldbuilding cards with customizable fields
- [ ] Implement worldbuilding trait visibility controls
- [ ] Build hierarchical worldbuilding organization
- [ ] Add worldbuilding templates (locations, cultures, magic systems)
- [ ] Create worldbuilding relationship mapping
- [ ] Implement worldbuilding generation from prompts
- [ ] Add worldbuilding consistency validation
- [ ] Build worldbuilding export capabilities

### Week 14: Outline & Scenes System
- [ ] Create outline system with unlimited chapter length
- [ ] Implement Acts/dividers (Part, Book, Episode, Section)
- [ ] Build document linking from outline chapters
- [ ] Add automatic document creation from chapters
- [ ] Implement outline updating from written documents
- [ ] Create Scenes & Draft building blocks
- [ ] Add scene validation with quick fixes
- [ ] Build word count and credit estimates for scenes

### Week 15: Series Support & Integration
- [ ] Implement series-level Story Bible sharing
- [ ] Create series timeline management
- [ ] Build cross-project data synchronization
- [ ] Add series consistency checking
- [ ] Implement Story Bible detection in text
- [ ] Create underlined element highlighting
- [ ] Build Saliency Engine for intelligent AI context
- [ ] Add visibility settings for spoiler management

## Story Bible Architecture

### Core Data Models
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryBible {
    pub id: i32,
    pub project_id: i32,
    pub braindump: Option<String>,
    pub synopsis: Option<String>,
    pub genre: Option<String>,
    pub style: Option<String>,
    pub style_examples: Option<String>,
    pub pov_mode: POVMode,
    pub global_pov: Option<String>,
    pub global_tense: Option<String>,
    pub global_character_pov_ids: Vec<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum POVMode {
    Global,
    PerChapter,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: i32,
    pub project_id: Option<i32>,
    pub series_id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub character_type: Option<String>,
    pub traits: HashMap<String, String>,
    pub is_visible: bool,
    pub original_project_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldElement {
    pub id: i32,
    pub project_id: Option<i32>,
    pub series_id: Option<i32>,
    pub name: String,
    pub element_type: String,
    pub description: Option<String>,
    pub properties: HashMap<String, String>,
    pub is_visible: bool,
    pub original_project_id: Option<i32>,
}
```

### Saliency Engine
```rust
pub struct SaliencyEngine {
    embedding_service: Arc<EmbeddingService>,
    relevance_calculator: RelevanceCalculator,
}

impl SaliencyEngine {
    pub async fn select_relevant_elements(&self,
        context: &str,
        story_bible: &StoryBible,
        max_elements: usize
    ) -> Result<Vec<StoryBibleElement>> {
        // Generate embedding for current context
        let context_embedding = self.embedding_service
            .generate_embedding(context)
            .await?;
        
        // Calculate relevance scores for all Story Bible elements
        let mut scored_elements = Vec::new();
        
        // Score characters
        for character in &story_bible.characters {
            if character.is_visible {
                let score = self.relevance_calculator
                    .calculate_character_relevance(&context_embedding, character)
                    .await?;
                scored_elements.push((StoryBibleElement::Character(character.clone()), score));
            }
        }
        
        // Score worldbuilding elements
        for element in &story_bible.worldbuilding {
            if element.is_visible {
                let score = self.relevance_calculator
                    .calculate_worldbuilding_relevance(&context_embedding, element)
                    .await?;
                scored_elements.push((StoryBibleElement::Worldbuilding(element.clone()), score));
            }
        }
        
        // Sort by relevance and return top elements
        scored_elements.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        Ok(scored_elements.into_iter()
            .take(max_elements)
            .map(|(element, _)| element)
            .collect())
    }
}
```

## Database Schema Extensions

### Story Bible Tables
```sql
-- Story Bible Core
CREATE TABLE story_bible (
    id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL,
    braindump TEXT,
    synopsis TEXT,
    genre TEXT,
    style TEXT,
    style_examples TEXT,
    pov_mode TEXT DEFAULT 'global',
    global_pov TEXT DEFAULT '3rd Person Limited',
    global_tense TEXT DEFAULT 'Past',
    global_character_pov_ids JSON,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- Characters
CREATE TABLE characters (
    id INTEGER PRIMARY KEY,
    project_id INTEGER,
    series_id INTEGER,
    name TEXT NOT NULL,
    description TEXT,
    character_type TEXT,
    traits JSON,
    is_visible BOOLEAN DEFAULT TRUE,
    original_project_id INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id),
    FOREIGN KEY (series_id) REFERENCES series(id),
    FOREIGN KEY (original_project_id) REFERENCES projects(id)
);

CREATE TABLE character_traits (
    id INTEGER PRIMARY KEY,
    character_id INTEGER NOT NULL,
    trait_name TEXT NOT NULL,
    trait_value TEXT,
    is_visible BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id)
);

-- Worldbuilding
CREATE TABLE worldbuilding (
    id INTEGER PRIMARY KEY,
    project_id INTEGER,
    series_id INTEGER,
    name TEXT NOT NULL,
    type TEXT,
    description TEXT,
    properties JSON,
    is_visible BOOLEAN DEFAULT TRUE,
    original_project_id INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id),
    FOREIGN KEY (series_id) REFERENCES series(id),
    FOREIGN KEY (original_project_id) REFERENCES projects(id)
);

-- Outlines with Acts
CREATE TABLE outlines (
    id INTEGER PRIMARY KEY,
    project_id INTEGER REFERENCES projects(id),
    chapter_number INTEGER,
    title TEXT,
    summary TEXT,
    pov TEXT,
    tense TEXT,
    character_pov_ids JSON,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE outline_acts (
    id INTEGER PRIMARY KEY,
    outline_id INTEGER NOT NULL,
    act_type TEXT NOT NULL, -- 'Part', 'Book', 'Episode', 'Section'
    act_number INTEGER NOT NULL,
    title TEXT NOT NULL,
    position INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (outline_id) REFERENCES outlines(id)
);

-- Scenes and Draft
CREATE TABLE scenes (
    id INTEGER PRIMARY KEY,
    outline_id INTEGER NOT NULL,
    scene_number INTEGER NOT NULL,
    title TEXT,
    summary TEXT,
    extra_instructions TEXT,
    pov TEXT,
    tense TEXT,
    character_pov_ids JSON,
    word_count_estimate INTEGER,
    credit_estimate INTEGER,
    is_validated BOOLEAN DEFAULT FALSE,
    validation_issues JSON,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (outline_id) REFERENCES outlines(id)
);
```

## Frontend Components

### Story Bible Interface
```typescript
interface StoryBibleProps {
  projectId: number;
  seriesId?: number;
}

export const StoryBible: React.FC<StoryBibleProps> = ({ projectId, seriesId }) => {
  const [activeTab, setActiveTab] = useState<'braindump' | 'characters' | 'worldbuilding' | 'outline'>('braindump');
  const { data: storyBible, isLoading } = useQuery(['storyBible', projectId], 
    () => fetchStoryBible(projectId)
  );

  return (
    <div className="story-bible-container">
      <div className="story-bible-tabs">
        <button 
          className={activeTab === 'braindump' ? 'active' : ''}
          onClick={() => setActiveTab('braindump')}
        >
          Braindump
        </button>
        <button 
          className={activeTab === 'characters' ? 'active' : ''}
          onClick={() => setActiveTab('characters')}
        >
          Characters
        </button>
        <button 
          className={activeTab === 'worldbuilding' ? 'active' : ''}
          onClick={() => setActiveTab('worldbuilding')}
        >
          Worldbuilding
        </button>
        <button 
          className={activeTab === 'outline' ? 'active' : ''}
          onClick={() => setActiveTab('outline')}
        >
          Outline
        </button>
      </div>

      <div className="story-bible-content">
        {activeTab === 'braindump' && (
          <BraindumpEditor 
            projectId={projectId}
            content={storyBible?.braindump}
            onUpdate={handleBraindumpUpdate}
          />
        )}
        {activeTab === 'characters' && (
          <CharactersManager 
            projectId={projectId}
            seriesId={seriesId}
            characters={storyBible?.characters}
          />
        )}
        {activeTab === 'worldbuilding' && (
          <WorldbuildingManager 
            projectId={projectId}
            seriesId={seriesId}
            elements={storyBible?.worldbuilding}
          />
        )}
        {activeTab === 'outline' && (
          <OutlineManager 
            projectId={projectId}
            outline={storyBible?.outline}
          />
        )}
      </div>
    </div>
  );
};
```

### Character Management
```typescript
interface CharacterCardProps {
  character: Character;
  onUpdate: (character: Character) => void;
  onDelete: (id: number) => void;
}

export const CharacterCard: React.FC<CharacterCardProps> = ({
  character,
  onUpdate,
  onDelete
}) => {
  const [isExpanded, setIsExpanded] = useState(false);
  const [traits, setTraits] = useState(character.traits);

  const handleTraitUpdate = (traitName: string, value: string, isVisible: boolean) => {
    const updatedTraits = {
      ...traits,
      [traitName]: { value, isVisible }
    };
    setTraits(updatedTraits);
    onUpdate({ ...character, traits: updatedTraits });
  };

  return (
    <div className="character-card">
      <div className="character-header">
        <h3>{character.name}</h3>
        <div className="character-actions">
          <button onClick={() => setIsExpanded(!isExpanded)}>
            {isExpanded ? 'Collapse' : 'Expand'}
          </button>
          <button onClick={() => onDelete(character.id)}>Delete</button>
        </div>
      </div>

      {isExpanded && (
        <div className="character-details">
          <div className="character-description">
            <label>Description:</label>
            <textarea 
              value={character.description || ''}
              onChange={(e) => onUpdate({ ...character, description: e.target.value })}
            />
          </div>

          <div className="character-traits">
            <h4>Traits</h4>
            {Object.entries(traits).map(([traitName, traitData]) => (
              <div key={traitName} className="trait-row">
                <input 
                  type="text" 
                  value={traitName}
                  onChange={(e) => handleTraitRename(traitName, e.target.value)}
                />
                <textarea 
                  value={traitData.value}
                  onChange={(e) => handleTraitUpdate(traitName, e.target.value, traitData.isVisible)}
                />
                <label>
                  <input 
                    type="checkbox"
                    checked={traitData.isVisible}
                    onChange={(e) => handleTraitUpdate(traitName, traitData.value, e.target.checked)}
                  />
                  Visible to AI
                </label>
              </div>
            ))}
            <button onClick={handleAddTrait}>Add Trait</button>
          </div>
        </div>
      )}
    </div>
  );
};
```

## Smart Import System

### Character Extraction
```rust
pub struct CharacterExtractor {
    ai_provider: Arc<dyn AIProvider>,
    nlp_processor: NLPProcessor,
}

impl CharacterExtractor {
    pub async fn extract_characters(&self, 
        content: &str, 
        max_characters: usize
    ) -> Result<Vec<Character>> {
        // Validate content length
        let word_count = self.count_words(content);
        if word_count > 60_000 {
            return Err(ExtractionError::ContentTooLarge);
        }

        // Use AI to identify and extract character information
        let extraction_prompt = format!(
            "Analyze this text and extract up to {} main characters. For each character, provide:\n\
            - Name\n\
            - Description\n\
            - Key traits\n\
            - Role in story\n\n\
            Text: {}",
            max_characters, content
        );

        let ai_response = self.ai_provider
            .generate_text(&extraction_prompt, &AIContext::default())
            .await?;

        // Parse AI response into character objects
        let characters = self.parse_character_response(&ai_response)?;
        
        // Limit to max_characters and 30 total
        let limited_characters = characters.into_iter()
            .take(max_characters.min(30))
            .collect();

        Ok(limited_characters)
    }

    pub async fn import_from_csv(&self, csv_content: &str) -> Result<Vec<Character>> {
        let mut reader = csv::Reader::from_reader(csv_content.as_bytes());
        let mut characters = Vec::new();

        for result in reader.records() {
            let record = result?;
            let character = self.parse_csv_record(&record)?;
            characters.push(character);
        }

        Ok(characters)
    }
}
```

## Success Criteria
- [ ] Story Bible foundation (Braindump, Synopsis, Genre, Style) fully functional
- [ ] Characters system with traits and visibility controls works correctly
- [ ] Worldbuilding system supports customizable cards and templates
- [ ] Outline system with Acts/dividers and document linking operational
- [ ] Scenes & Draft functionality with validation and estimates
- [ ] Series support enables cross-project Story Bible sharing
- [ ] POV and Tense settings work globally and per-chapter
- [ ] Story Bible detection highlights elements in text
- [ ] Smart import extracts characters from text and CSV files
- [ ] Saliency Engine intelligently selects relevant context for AI

## Risk Mitigation
- **Data Complexity**: Implement robust validation and error handling
- **Performance**: Optimize database queries and caching for large Story Bibles
- **Series Synchronization**: Handle conflicts and maintain data consistency
- **AI Context Limits**: Intelligent truncation and prioritization of Story Bible data
- **Import Accuracy**: Validation and user review of extracted data

## Dependencies
### Rust
- csv = "1.3"
- serde_json = "1.0"
- chrono = { version = "0.4", features = ["serde"] }
- uuid = { version = "1.0", features = ["v4"] }
- lancedb = "0.4" # For embeddings and similarity search

### Frontend
- react-hook-form = "^7.47.0"
- @radix-ui/react-tabs = "^1.0.0"
- @radix-ui/react-accordion = "^1.0.0"
- react-dnd = "^16.0.0"
- react-dnd-html5-backend = "^16.0.0"

## Next Phase
Phase 4 will focus on advanced AI features including multiple models, prose modes, and specialized tools like Visualize and advanced brainstorming.
