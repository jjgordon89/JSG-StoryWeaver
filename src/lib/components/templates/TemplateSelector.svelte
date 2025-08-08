<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { Button } from '$lib/components/ui/button';
  import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/components/ui/card';
  import { Badge } from '$lib/components/ui/badge';
  import { ScrollArea } from '$lib/components/ui/scroll-area';
  import { Separator } from '$lib/components/ui/separator';
  import { ChevronRight, Sparkles, User, Globe } from 'lucide-svelte';
  import type { CharacterTemplate, WorldBuildingTemplate } from '$lib/types/templates';

  export let templates: (CharacterTemplate | WorldBuildingTemplate)[] = [];
  export let templateType: 'character' | 'worldbuilding' = 'character';
  export let selectedTemplateId: string | null = null;
  export let isLoading = false;

  const dispatch = createEventDispatcher<{
    select: { templateId: string; template: CharacterTemplate | WorldBuildingTemplate };
    apply: { templateId: string; template: CharacterTemplate | WorldBuildingTemplate };
  }>();

  function selectTemplate(template: CharacterTemplate | WorldBuildingTemplate) {
    selectedTemplateId = template.id;
    dispatch('select', { templateId: template.id, template });
  }

  function applyTemplate(template: CharacterTemplate | WorldBuildingTemplate) {
    dispatch('apply', { templateId: template.id, template });
  }

  function getTemplateIcon(template: CharacterTemplate | WorldBuildingTemplate) {
    if (templateType === 'character') {
      return User;
    } else {
      const worldTemplate = template as WorldBuildingTemplate;
      switch (worldTemplate.element_type) {
        case 'location':
          return Globe;
        case 'organization':
          return User;
        case 'culture':
          return Sparkles;
        case 'magic':
          return Sparkles;
        case 'technology':
          return ChevronRight;
        case 'artifact':
          return Sparkles;
        default:
          return Globe;
      }
    }
  }

  function getArchetypeOrType(template: CharacterTemplate | WorldBuildingTemplate): string {
    if (templateType === 'character') {
      return (template as CharacterTemplate).archetype;
    } else {
      return (template as WorldBuildingTemplate).element_type;
    }
  }

  function getDefaultItems(template: CharacterTemplate | WorldBuildingTemplate): string[] {
    if (templateType === 'character') {
      const charTemplate = template as CharacterTemplate;
      return charTemplate.default_traits.slice(0, 3).map(trait => trait.trait_name);
    } else {
      const worldTemplate = template as WorldBuildingTemplate;
      return worldTemplate.default_properties.slice(0, 3).map(prop => prop.property_name);
    }
  }
</script>

<div class="template-selector">
  {#if isLoading}
    <div class="flex items-center justify-center p-8">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
      <span class="ml-2 text-muted-foreground">Loading templates...</span>
    </div>
  {:else if templates.length === 0}
    <div class="text-center p-8 text-muted-foreground">
      <Sparkles class="h-12 w-12 mx-auto mb-4 opacity-50" />
      <p>No templates available</p>
    </div>
  {:else}
    <ScrollArea class="h-96">
      <div class="space-y-3 p-1">
        {#each templates as template (template.id)}
          <Card 
            class="cursor-pointer transition-all hover:shadow-md {selectedTemplateId === template.id ? 'ring-2 ring-primary' : ''}"
            on:click={() => selectTemplate(template)}
          >
            <CardHeader class="pb-3">
              <div class="flex items-start justify-between">
                <div class="flex items-center space-x-2">
                  <svelte:component this={getTemplateIcon(template)} class="h-5 w-5 text-primary" />
                  <div>
                    <CardTitle class="text-base">{template.name}</CardTitle>
                    <div class="flex items-center space-x-2 mt-1">
                      <Badge variant="secondary" class="text-xs">
                        {getArchetypeOrType(template)}
                      </Badge>
                      {#if template.is_system}
                        <Badge variant="outline" class="text-xs">
                          System
                        </Badge>
                      {/if}
                    </div>
                  </div>
                </div>
                <Button
                  size="sm"
                  variant="ghost"
                  class="h-8 w-8 p-0"
                  on:click|stopPropagation={() => applyTemplate(template)}
                >
                  <ChevronRight class="h-4 w-4" />
                </Button>
              </div>
            </CardHeader>
            <CardContent class="pt-0">
              <CardDescription class="text-sm mb-3">
                {template.description}
              </CardDescription>
              
              <div class="space-y-2">
                <div class="text-xs font-medium text-muted-foreground">
                  {templateType === 'character' ? 'Default Traits:' : 'Default Properties:'}
                </div>
                <div class="flex flex-wrap gap-1">
                  {#each getDefaultItems(template) as item}
                    <Badge variant="outline" class="text-xs">
                      {item}
                    </Badge>
                  {/each}
                  {#if (templateType === 'character' ? (template as CharacterTemplate).default_traits.length : (template as WorldBuildingTemplate).default_properties.length) > 3}
                    <Badge variant="outline" class="text-xs">
                      +{(templateType === 'character' ? (template as CharacterTemplate).default_traits.length : (template as WorldBuildingTemplate).default_properties.length) - 3} more
                    </Badge>
                  {/if}
                </div>
              </div>
            </CardContent>
          </Card>
        {/each}
      </div>
    </ScrollArea>
  {/if}
</div>

<style>
  .template-selector {
    @apply w-full;
  }
</style>