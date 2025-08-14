import { OutlineTemplateType } from '../types/canvas';

export interface TemplateStructure {
  elements: {
    type: string;
    title: string;
    content?: string;
    x: number;
    y: number;
    width?: number;
    height?: number;
    color?: string;
    metadata?: any;
    connections?: number[];
  }[];
}

export const defaultTemplates: {
  name: string;
  description: string;
  template_type: OutlineTemplateType;
  structure: TemplateStructure;
}[] = [
  {
    name: "Hero's Journey",
    description: "The classic monomyth structure following a hero's transformative adventure through 12 stages.",
    template_type: 'heros_journey',
    structure: {
      elements: [
        { type: 'act', title: 'Ordinary World', content: 'The hero\'s normal life before transformation', x: 100, y: 100, width: 200, height: 120, color: '#e3f2fd' },
        { type: 'plot_point', title: 'Call to Adventure', content: 'The inciting incident that starts the journey', x: 350, y: 100, width: 200, height: 120, color: '#fff3e0' },
        { type: 'conflict', title: 'Refusal of the Call', content: 'Initial hesitation or fear', x: 600, y: 100, width: 200, height: 120, color: '#ffebee' },
        { type: 'character_arc', title: 'Meeting the Mentor', content: 'Wise figure provides advice/magical gifts', x: 850, y: 100, width: 200, height: 120, color: '#f3e5f5' },
        { type: 'act', title: 'Crossing the Threshold', content: 'Committing to the adventure', x: 100, y: 280, width: 200, height: 120, color: '#e8f5e8' },
        { type: 'scene', title: 'Tests, Allies, Enemies', content: 'Challenges and character development', x: 350, y: 280, width: 200, height: 120, color: '#fff8e1' },
        { type: 'plot_point', title: 'Approach to the Inmost Cave', content: 'Preparing for the major challenge', x: 600, y: 280, width: 200, height: 120, color: '#fce4ec' },
        { type: 'conflict', title: 'The Ordeal', content: 'The crisis point of the adventure', x: 850, y: 280, width: 200, height: 120, color: '#ffebee' },
        { type: 'theme', title: 'The Reward', content: 'Surviving and gaining something', x: 100, y: 460, width: 200, height: 120, color: '#e0f2f1' },
        { type: 'plot_point', title: 'The Road Back', content: 'Beginning the journey home', x: 350, y: 460, width: 200, height: 120, color: '#fff3e0' },
        { type: 'conflict', title: 'Resurrection', content: 'Final test and transformation', x: 600, y: 460, width: 200, height: 120, color: '#ffebee' },
        { type: 'act', title: 'Return with the Elixir', content: 'Coming home changed with wisdom', x: 850, y: 460, width: 200, height: 120, color: '#e3f2fd' }
      ]
    }
  },
  {
    name: "Three Act Structure",
    description: "Traditional three-act structure with setup, confrontation, and resolution.",
    template_type: 'three_act',
    structure: {
      elements: [
        { type: 'act', title: 'Act I: Setup', content: 'Introduce characters, world, and conflict', x: 100, y: 100, width: 300, height: 150, color: '#e3f2fd' },
        { type: 'plot_point', title: 'Inciting Incident', content: 'Event that starts the main story', x: 450, y: 100, width: 200, height: 100, color: '#fff3e0' },
        { type: 'plot_point', title: 'Plot Point 1', content: 'End of Act I, major turning point', x: 700, y: 100, width: 200, height: 100, color: '#ffebee' },
        { type: 'act', title: 'Act II: Confrontation', content: 'Rising action, obstacles, character development', x: 100, y: 300, width: 600, height: 150, color: '#fff8e1' },
        { type: 'plot_point', title: 'Midpoint', content: 'Major revelation or setback', x: 750, y: 300, width: 200, height: 100, color: '#f3e5f5' },
        { type: 'conflict', title: 'Crisis', content: 'Lowest point, all seems lost', x: 100, y: 500, width: 200, height: 100, color: '#ffebee' },
        { type: 'plot_point', title: 'Plot Point 2', content: 'End of Act II, final push begins', x: 350, y: 500, width: 200, height: 100, color: '#fff3e0' },
        { type: 'act', title: 'Act III: Resolution', content: 'Climax and resolution of conflicts', x: 600, y: 500, width: 300, height: 150, color: '#e0f2f1' }
      ]
    }
  },
  {
    name: "Save the Cat Beat Sheet",
    description: "Blake Snyder's 15-beat structure for screenwriting and storytelling.",
    template_type: 'save_the_cat',
    structure: {
      elements: [
        { type: 'scene', title: 'Opening Image', content: 'Visual that represents the story', x: 50, y: 50, width: 180, height: 100, color: '#e3f2fd' },
        { type: 'scene', title: 'Theme Stated', content: 'What the story is about', x: 250, y: 50, width: 180, height: 100, color: '#f3e5f5' },
        { type: 'scene', title: 'Set-Up', content: 'Introduce hero and status quo', x: 450, y: 50, width: 180, height: 100, color: '#e8f5e8' },
        { type: 'plot_point', title: 'Catalyst', content: 'Life-changing event', x: 650, y: 50, width: 180, height: 100, color: '#fff3e0' },
        { type: 'conflict', title: 'Debate', content: 'Should I go?', x: 850, y: 50, width: 180, height: 100, color: '#ffebee' },
        { type: 'act', title: 'Break into Two', content: 'Leaving the old world', x: 50, y: 200, width: 180, height: 100, color: '#e0f2f1' },
        { type: 'scene', title: 'B Story', content: 'Subplot, usually love story', x: 250, y: 200, width: 180, height: 100, color: '#fce4ec' },
        { type: 'scene', title: 'Fun and Games', content: 'Promise of the premise', x: 450, y: 200, width: 180, height: 100, color: '#fff8e1' },
        { type: 'plot_point', title: 'Midpoint', content: 'False victory or defeat', x: 650, y: 200, width: 180, height: 100, color: '#f3e5f5' },
        { type: 'conflict', title: 'Bad Guys Close In', content: 'Forces of antagonism regroup', x: 850, y: 200, width: 180, height: 100, color: '#ffebee' },
        { type: 'conflict', title: 'All Is Lost', content: 'Lowest point', x: 50, y: 350, width: 180, height: 100, color: '#ffebee' },
        { type: 'theme', title: 'Dark Night of the Soul', content: 'Moment of despair', x: 250, y: 350, width: 180, height: 100, color: '#f3e5f5' },
        { type: 'act', title: 'Break into Three', content: 'Solution appears', x: 450, y: 350, width: 180, height: 100, color: '#e0f2f1' },
        { type: 'scene', title: 'Finale', content: 'Climax and resolution', x: 650, y: 350, width: 180, height: 100, color: '#e8f5e8' },
        { type: 'scene', title: 'Final Image', content: 'Opposite of opening image', x: 850, y: 350, width: 180, height: 100, color: '#e3f2fd' }
      ]
    }
  },
  {
    name: "Story Circle",
    description: "Dan Harmon's simplified version of the Hero's Journey in 8 steps.",
    template_type: 'story_circle',
    structure: {
      elements: [
        { type: 'act', title: '1. You (Comfort)', content: 'Character in familiar situation', x: 400, y: 50, width: 200, height: 100, color: '#e3f2fd' },
        { type: 'plot_point', title: '2. Need', content: 'Something is missing', x: 650, y: 150, width: 200, height: 100, color: '#fff3e0' },
        { type: 'act', title: '3. Go (Unfamiliar)', content: 'Enter new situation', x: 650, y: 350, width: 200, height: 100, color: '#ffebee' },
        { type: 'scene', title: '4. Search', content: 'Adapt to new environment', x: 400, y: 450, width: 200, height: 100, color: '#fff8e1' },
        { type: 'conflict', title: '5. Find', content: 'Find what they needed', x: 150, y: 450, width: 200, height: 100, color: '#e8f5e8' },
        { type: 'conflict', title: '6. Take', content: 'Pay heavy price for it', x: 50, y: 350, width: 200, height: 100, color: '#f3e5f5' },
        { type: 'act', title: '7. Return (Familiar)', content: 'Return to familiar', x: 50, y: 150, width: 200, height: 100, color: '#e0f2f1' },
        { type: 'theme', title: '8. Change', content: 'Now capable of change', x: 150, y: 50, width: 200, height: 100, color: '#fce4ec' }
      ]
    }
  },
  {
    name: "Romance Outline",
    description: "Structure specifically designed for romance novels and stories.",
    template_type: 'romance_outline',
    structure: {
      elements: [
        { type: 'scene', title: 'Meet Cute', content: 'First meeting between love interests', x: 100, y: 100, width: 200, height: 120, color: '#fce4ec' },
        { type: 'character_arc', title: 'Initial Attraction', content: 'Spark of interest despite obstacles', x: 350, y: 100, width: 200, height: 120, color: '#f3e5f5' },
        { type: 'conflict', title: 'The Hook', content: 'Reason they must be together', x: 600, y: 100, width: 200, height: 120, color: '#fff3e0' },
        { type: 'scene', title: 'Getting to Know You', content: 'Building relationship and tension', x: 100, y: 280, width: 200, height: 120, color: '#e8f5e8' },
        { type: 'plot_point', title: 'First Kiss/Intimacy', content: 'Physical and emotional connection', x: 350, y: 280, width: 200, height: 120, color: '#fce4ec' },
        { type: 'conflict', title: 'The Barrier', content: 'Major obstacle to relationship', x: 600, y: 280, width: 200, height: 120, color: '#ffebee' },
        { type: 'conflict', title: 'The Crisis', content: 'Relationship seems impossible', x: 100, y: 460, width: 200, height: 120, color: '#ffebee' },
        { type: 'theme', title: 'The Epiphany', content: 'Realization about love/self', x: 350, y: 460, width: 200, height: 120, color: '#f3e5f5' },
        { type: 'scene', title: 'Grand Gesture', content: 'Proving love and commitment', x: 600, y: 460, width: 200, height: 120, color: '#e0f2f1' },
        { type: 'act', title: 'Happily Ever After', content: 'Resolution and commitment', x: 350, y: 640, width: 200, height: 120, color: '#fce4ec' }
      ]
    }
  },
  {
    name: "Seven Point Story Structure",
    description: "Dan Wells' structure focusing on character development and plot progression.",
    template_type: 'seven_point',
    structure: {
      elements: [
        { type: 'act', title: 'Hook', content: 'Opening that grabs attention', x: 100, y: 100, width: 200, height: 120, color: '#e3f2fd' },
        { type: 'plot_point', title: 'Plot Turn 1', content: 'Call to adventure', x: 350, y: 100, width: 200, height: 120, color: '#fff3e0' },
        { type: 'scene', title: 'Pinch Point 1', content: 'Pressure from antagonist', x: 600, y: 100, width: 200, height: 120, color: '#ffebee' },
        { type: 'conflict', title: 'Midpoint', content: 'Major revelation/setback', x: 350, y: 280, width: 200, height: 120, color: '#f3e5f5' },
        { type: 'scene', title: 'Pinch Point 2', content: 'Antagonist shows full power', x: 100, y: 460, width: 200, height: 120, color: '#ffebee' },
        { type: 'plot_point', title: 'Plot Turn 2', content: 'Final piece of puzzle', x: 350, y: 460, width: 200, height: 120, color: '#fff3e0' },
        { type: 'act', title: 'Resolution', content: 'Climax and ending', x: 600, y: 460, width: 200, height: 120, color: '#e0f2f1' }
      ]
    }
  }
];

export const createDefaultTemplates = async () => {
  const { invoke } = await import('@tauri-apps/api/tauri');
  
  for (const template of defaultTemplates) {
    try {
      await invoke('create_outline_template', {
        name: template.name,
        description: template.description,
        templateType: template.template_type,
        structure: JSON.stringify(template.structure),
        isOfficial: true
      });
    } catch (error) {
      // Template might already exist, which is fine
      console.log(`Template "${template.name}" might already exist:`, error);
    }
  }
};
