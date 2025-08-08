// Template system types for StoryWeaver

export interface CharacterTemplateTrait {
  trait_type: string;
  trait_name: string;
  default_value: string;
  is_required: boolean;
  description: string;
}

export interface CharacterTemplate {
  id: string;
  name: string;
  description: string;
  archetype: string;
  default_traits: CharacterTemplateTrait[];
  is_system: boolean;
  created_at: string;
}

export interface WorldBuildingTemplateProperty {
  property_name: string;
  default_value?: string;
  is_required: boolean;
  description: string;
  property_type: string; // "text", "number", "boolean", "list"
}

export interface WorldBuildingTemplate {
  id: string;
  name: string;
  description: string;
  element_type: string;
  default_properties: WorldBuildingTemplateProperty[];
  is_system: boolean;
  created_at: string;
}

// Template application request types
export interface ApplyCharacterTemplateRequest {
  template_id: string;
  project_id: string;
  name: string;
  description?: string;
  trait_overrides?: Record<string, string>;
}

export interface ApplyWorldBuildingTemplateRequest {
  template_id: string;
  project_id: string;
  name: string;
  description?: string;
  property_overrides?: Record<string, any>;
}

// Template service functions
export class TemplateService {
  // Character template methods
  static async getCharacterTemplates(): Promise<CharacterTemplate[]> {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke('get_character_templates');
  }

  static async getCharacterTemplatesByArchetype(archetype: string): Promise<CharacterTemplate[]> {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke('get_character_templates_by_archetype', { archetype });
  }

  static async getCharacterArchetypes(): Promise<string[]> {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke('get_character_archetypes');
  }

  static async applyCharacterTemplate(request: ApplyCharacterTemplateRequest): Promise<string> {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke('apply_character_template', {
      templateId: request.template_id,
      projectId: request.project_id,
      name: request.name,
      description: request.description,
      traitOverrides: request.trait_overrides,
    });
  }

  // Worldbuilding template methods
  static async getWorldBuildingTemplates(): Promise<WorldBuildingTemplate[]> {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke('get_worldbuilding_templates');
  }

  static async getWorldBuildingTemplatesByType(elementType: string): Promise<WorldBuildingTemplate[]> {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke('get_worldbuilding_templates_by_type', { elementType });
  }

  static async getWorldBuildingElementTypes(): Promise<string[]> {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke('get_worldbuilding_element_types');
  }

  static async applyWorldBuildingTemplate(request: ApplyWorldBuildingTemplateRequest): Promise<string> {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke('apply_worldbuilding_template', {
      templateId: request.template_id,
      projectId: request.project_id,
      name: request.name,
      description: request.description,
      propertyOverrides: request.property_overrides,
    });
  }
}