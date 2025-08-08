<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { Button } from '$lib/components/ui/button';
  import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from '$lib/components/ui/dialog';
  import { Input } from '$lib/components/ui/input';
  import { Label } from '$lib/components/ui/label';
  import { Textarea } from '$lib/components/ui/textarea';
  import { Checkbox } from '$lib/components/ui/checkbox';
  import { Badge } from '$lib/components/ui/badge';
  import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/components/ui/card';
  import { Separator } from '$lib/components/ui/separator';
  import { ScrollArea } from '$lib/components/ui/scroll-area';
  import { Sparkles, User, Globe } from 'lucide-svelte';
  import type { CharacterTemplate, WorldBuildingTemplate, ApplyCharacterTemplateRequest, ApplyWorldBuildingTemplateRequest } from '$lib/types/templates';

  export let open = false;
  export let template: CharacterTemplate | WorldBuildingTemplate | null = null;
  export let templateType: 'character' | 'worldbuilding' = 'character';
  export let projectId: string;
  export let isApplying = false;

  const dispatch = createEventDispatcher<{
    apply: { request: ApplyCharacterTemplateRequest | ApplyWorldBuildingTemplateRequest };
    cancel: void;
  }>();

  let name = '';
  let description = '';
  let overrides: Record<string, any> = {};
  let selectedProperties: Record<string, boolean> = {};

  $: if (template && open) {
    resetForm();
  }

  function resetForm() {
    name = '';
    description = '';
    overrides = {};
    selectedProperties = {};
    
    if (template) {
      if (templateType === 'character') {
        const charTemplate = template as CharacterTemplate;
        charTemplate.default_traits.forEach(trait => {
          selectedProperties[trait.trait_name] = trait.is_required;
          if (trait.default_value) {
            overrides[trait.trait_name] = trait.default_value;
          }
        });
      } else {
        const worldTemplate = template as WorldBuildingTemplate;
        worldTemplate.default_properties.forEach(prop => {
          selectedProperties[prop.property_name] = prop.is_required;
          if (prop.default_value) {
            overrides[prop.property_name] = prop.default_value;
          }
        });
      }
    }
  }

  function handleApply() {
    if (!template || !name.trim()) return;

    // Filter overrides to only include selected properties
    const filteredOverrides: Record<string, any> = {};
    Object.keys(selectedProperties).forEach(key => {
      if (selectedProperties[key] && overrides[key] !== undefined) {
        filteredOverrides[key] = overrides[key];
      }
    });

    if (templateType === 'character') {
      const request: ApplyCharacterTemplateRequest = {
        template_id: template.id,
        project_id: projectId,
        name: name.trim(),
        description: description.trim() || undefined,
        trait_overrides: Object.keys(filteredOverrides).length > 0 ? filteredOverrides : undefined,
      };
      dispatch('apply', { request });
    } else {
      const request: ApplyWorldBuildingTemplateRequest = {
        template_id: template.id,
        project_id: projectId,
        name: name.trim(),
        description: description.trim() || undefined,
        property_overrides: Object.keys(filteredOverrides).length > 0 ? filteredOverrides : undefined,
      };
      dispatch('apply', { request });
    }
  }

  function handleCancel() {
    dispatch('cancel');
  }

  function getTemplateIcon() {
    if (!template) return Sparkles;
    
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
          return Sparkles;
        case 'artifact':
          return Sparkles;
        default:
          return Globe;
      }
    }
  }

  function getItems() {
    if (!template) return [];
    
    if (templateType === 'character') {
      return (template as CharacterTemplate).default_traits;
    } else {
      return (template as WorldBuildingTemplate).default_properties;
    }
  }

  function getItemName(item: any): string {
    return templateType === 'character' ? item.trait_name : item.property_name;
  }

  function getItemValue(item: any): string {
    return templateType === 'character' ? item.default_value : item.default_value;
  }

  function getItemType(item: any): string {
    return templateType === 'character' ? item.trait_type : item.property_type;
  }

  function isRequired(item: any): boolean {
    return item.is_required;
  }
</script>

<Dialog bind:open>
  <DialogContent class="max-w-2xl max-h-[90vh] overflow-hidden flex flex-col">
    <DialogHeader>
      <div class="flex items-center space-x-2">
        <svelte:component this={getTemplateIcon()} class="h-5 w-5 text-primary" />
        <DialogTitle>Apply Template: {template?.name || ''}</DialogTitle>
      </div>
      <DialogDescription>
        {template?.description || ''}
      </DialogDescription>
    </DialogHeader>

    <div class="flex-1 overflow-hidden">
      <ScrollArea class="h-full pr-4">
        <div class="space-y-6">
          <!-- Basic Information -->
          <div class="space-y-4">
            <div class="space-y-2">
              <Label for="name">Name *</Label>
              <Input
                id="name"
                bind:value={name}
                placeholder={`Enter ${templateType} name`}
                required
              />
            </div>
            
            <div class="space-y-2">
              <Label for="description">Description</Label>
              <Textarea
                id="description"
                bind:value={description}
                placeholder="Optional description"
                rows={2}
              />
            </div>
          </div>

          <Separator />

          <!-- Template Properties/Traits -->
          <div class="space-y-4">
            <div class="flex items-center space-x-2">
              <h3 class="text-lg font-semibold">
                {templateType === 'character' ? 'Character Traits' : 'Element Properties'}
              </h3>
              <Badge variant="secondary" class="text-xs">
                {getItems().length} items
              </Badge>
            </div>
            
            <div class="space-y-3">
              {#each getItems() as item}
                <Card class="p-4">
                  <div class="space-y-3">
                    <div class="flex items-center justify-between">
                      <div class="flex items-center space-x-2">
                        <Checkbox
                          bind:checked={selectedProperties[getItemName(item)]}
                          disabled={isRequired(item)}
                        />
                        <Label class="font-medium">
                          {getItemName(item)}
                          {#if isRequired(item)}
                            <span class="text-red-500">*</span>
                          {/if}
                        </Label>
                        <Badge variant="outline" class="text-xs">
                          {getItemType(item)}
                        </Badge>
                      </div>
                    </div>
                    
                    <p class="text-sm text-muted-foreground">
                      {item.description}
                    </p>
                    
                    {#if selectedProperties[getItemName(item)]}
                      <div class="space-y-2">
                        <Label for={getItemName(item)} class="text-sm">
                          Value
                        </Label>
                        {#if getItemType(item) === 'text' || getItemType(item) === 'list'}
                          <Textarea
                            id={getItemName(item)}
                            bind:value={overrides[getItemName(item)]}
                            placeholder={getItemValue(item) || `Enter ${getItemName(item)}`}
                            rows={2}
                          />
                        {:else if getItemType(item) === 'number'}
                          <Input
                            id={getItemName(item)}
                            type="number"
                            bind:value={overrides[getItemName(item)]}
                            placeholder={getItemValue(item) || '0'}
                          />
                        {:else if getItemType(item) === 'boolean'}
                          <Checkbox
                            bind:checked={overrides[getItemName(item)]}
                          />
                        {:else}
                          <Input
                            id={getItemName(item)}
                            bind:value={overrides[getItemName(item)]}
                            placeholder={getItemValue(item) || `Enter ${getItemName(item)}`}
                          />
                        {/if}
                      </div>
                    {/if}
                  </div>
                </Card>
              {/each}
            </div>
          </div>
        </div>
      </ScrollArea>
    </div>

    <DialogFooter>
      <Button variant="outline" on:click={handleCancel} disabled={isApplying}>
        Cancel
      </Button>
      <Button on:click={handleApply} disabled={!name.trim() || isApplying}>
        {#if isApplying}
          <div class="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"></div>
        {/if}
        Apply Template
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>