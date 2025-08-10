import React, { useState, useEffect } from 'react';
import { Button } from '../../../../components/ui/Button';
import { Card, CardContent, CardHeader, CardTitle } from '../../../../components/ui/Card';
import { Input } from '../../../../components/ui/input';
import { Textarea } from '../../../../components/ui/textarea';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../../../../components/ui/select';
import { Badge } from '../../../../components/ui/badge';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogTrigger } from '../../../../components/ui/dialog';
import { Tree, TreeNode } from '../../../../components/ui/tree';
import { 
  ChevronRight, 
  ChevronDown, 
  Plus, 
  Edit, 
  Trash2, 
  Move, 
  FolderPlus, 
  Folder, 
  FolderOpen,
  Globe,
  MapPin,
  Users,
  Crown,
  Sword,
  Book,
  Star,
  Settings,
  Search,
  Filter,
  Eye,
  EyeOff
} from 'lucide-react';

interface WorldElement {
  id: string;
  name: string;
  element_type: string;
  description: string;
  details?: string;
  significance?: string;
  visibility: 'always' | 'chapter' | 'never';
  series_shared: boolean;
  parent_id?: string;
  category_id?: string;
  order_index: number;
  properties: Array<{ property_name: string; property_value: string }>;
  tags: string[];
  relationships: Array<{
    target_id: string;
    relationship_type: string;
    description: string;
  }>;
  created_at: string;
  updated_at: string;
}

interface Category {
  id: string;
  name: string;
  description: string;
  icon: string;
  color: string;
  parent_id?: string;
  order_index: number;
  element_count: number;
  subcategories: Category[];
  elements: WorldElement[];
}

interface HierarchicalWorldbuildingProps {
  projectId: string;
  categories: Category[];
  elements: WorldElement[];
  onCreateCategory: (category: Omit<Category, 'id' | 'element_count' | 'subcategories' | 'elements'>) => Promise<void>;
  onUpdateCategory: (categoryId: string, updates: Partial<Category>) => Promise<void>;
  onDeleteCategory: (categoryId: string) => Promise<void>;
  onCreateElement: (element: Omit<WorldElement, 'id' | 'created_at' | 'updated_at'>) => Promise<void>;
  onUpdateElement: (elementId: string, updates: Partial<WorldElement>) => Promise<void>;
  onDeleteElement: (elementId: string) => Promise<void>;
  onMoveElement: (elementId: string, newCategoryId: string, newParentId?: string) => Promise<void>;
  onReorderElements: (categoryId: string, elementIds: string[]) => Promise<void>;
}

const ELEMENT_TYPE_ICONS = {
  location: MapPin,
  character: Users,
  organization: Crown,
  artifact: Star,
  event: Book,
  culture: Globe,
  technology: Settings,
  magic: Star,
  religion: Book,
  politics: Crown,
  military: Sword,
  custom: Settings
};

const CATEGORY_COLORS = [
  'bg-blue-500',
  'bg-green-500',
  'bg-purple-500',
  'bg-red-500',
  'bg-yellow-500',
  'bg-indigo-500',
  'bg-pink-500',
  'bg-gray-500'
];

const HierarchicalWorldbuilding: React.FC<HierarchicalWorldbuildingProps> = ({
  projectId,
  categories,
  elements,
  onCreateCategory,
  onUpdateCategory,
  onDeleteCategory,
  onCreateElement,
  onUpdateElement,
  onDeleteElement,
  onMoveElement,
  onReorderElements
}) => {
  const [expandedCategories, setExpandedCategories] = useState<Set<string>>(new Set());
  const [selectedCategory, setSelectedCategory] = useState<string | null>(null);
  const [selectedElement, setSelectedElement] = useState<WorldElement | null>(null);
  const [showCreateCategoryDialog, setShowCreateCategoryDialog] = useState(false);
  const [showCreateElementDialog, setShowCreateElementDialog] = useState(false);
  const [editingCategory, setEditingCategory] = useState<Category | null>(null);
  const [editingElement, setEditingElement] = useState<WorldElement | null>(null);
  const [searchQuery, setSearchQuery] = useState('');
  const [filterType, setFilterType] = useState('');
  const [filterVisibility, setFilterVisibility] = useState('');
  const [draggedElement, setDraggedElement] = useState<string | null>(null);
  const [view, setView] = useState<'tree' | 'grid' | 'list'>('tree');

  const [newCategory, setNewCategory] = useState<Partial<Category>>({
    name: '',
    description: '',
    icon: 'Folder',
    color: 'bg-blue-500',
    parent_id: undefined,
    order_index: 0
  });

  const [newElement, setNewElement] = useState<Partial<WorldElement>>({
    name: '',
    element_type: 'location',
    description: '',
    details: '',
    visibility: 'always',
    series_shared: false,
    parent_id: undefined,
    category_id: selectedCategory || undefined,
    order_index: 0,
    properties: [],
    tags: [],
    relationships: []
  });

  // Build hierarchical structure
  const buildHierarchy = (items: Category[], parentId?: string): Category[] => {
    return items
      .filter(item => item.parent_id === parentId)
      .sort((a, b) => a.order_index - b.order_index)
      .map(item => ({
        ...item,
        subcategories: buildHierarchy(items, item.id),
        elements: elements
          .filter(element => element.category_id === item.id && !element.parent_id)
          .sort((a, b) => a.order_index - b.order_index)
      }));
  };

  const hierarchicalCategories = buildHierarchy(categories);

  // Filter elements based on search and filters
  const filteredElements = elements.filter(element => {
    if (searchQuery && !element.name.toLowerCase().includes(searchQuery.toLowerCase()) &&
        !element.description.toLowerCase().includes(searchQuery.toLowerCase())) {
      return false;
    }
    if (filterType && element.element_type !== filterType) return false;
    if (filterVisibility && element.visibility !== filterVisibility) return false;
    return true;
  });

  const toggleCategory = (categoryId: string) => {
    const newExpanded = new Set(expandedCategories);
    if (newExpanded.has(categoryId)) {
      newExpanded.delete(categoryId);
    } else {
      newExpanded.add(categoryId);
    }
    setExpandedCategories(newExpanded);
  };

  const handleCreateCategory = async () => {
    if (!newCategory.name?.trim()) {
      alert('Please enter a category name');
      return;
    }

    try {
      await onCreateCategory({
        name: newCategory.name,
        description: newCategory.description || '',
        icon: newCategory.icon || 'Folder',
        color: newCategory.color || 'bg-blue-500',
        parent_id: newCategory.parent_id,
        order_index: newCategory.order_index || 0
      });
      
      setShowCreateCategoryDialog(false);
      setNewCategory({
        name: '',
        description: '',
        icon: 'Folder',
        color: 'bg-blue-500',
        parent_id: undefined,
        order_index: 0
      });
    } catch (error) {
      console.error('Failed to create category:', error);
      alert('Failed to create category');
    }
  };

  const handleCreateElement = async () => {
    if (!newElement.name?.trim()) {
      alert('Please enter an element name');
      return;
    }

    try {
      await onCreateElement({
        name: newElement.name,
        element_type: newElement.element_type || 'location',
        description: newElement.description || '',
        details: newElement.details || '',
        visibility: newElement.visibility || 'always',
        series_shared: newElement.series_shared || false,
        parent_id: newElement.parent_id,
        category_id: newElement.category_id || selectedCategory || undefined,
        order_index: newElement.order_index || 0,
        properties: newElement.properties || [],
        tags: newElement.tags || [],
        relationships: newElement.relationships || []
      });
      
      setShowCreateElementDialog(false);
      setNewElement({
        name: '',
        element_type: 'location',
        description: '',
        details: '',
        visibility: 'always',
        series_shared: false,
        parent_id: undefined,
        category_id: selectedCategory || undefined,
        order_index: 0,
        properties: [],
        tags: [],
        relationships: []
      });
    } catch (error) {
      console.error('Failed to create element:', error);
      alert('Failed to create element');
    }
  };

  const handleDragStart = (elementId: string) => {
    setDraggedElement(elementId);
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
  };

  const handleDrop = async (e: React.DragEvent, targetCategoryId: string) => {
    e.preventDefault();
    if (!draggedElement) return;

    try {
      await onMoveElement(draggedElement, targetCategoryId);
      setDraggedElement(null);
    } catch (error) {
      console.error('Failed to move element:', error);
      alert('Failed to move element');
    }
  };

  const getElementIcon = (elementType: string) => {
    const IconComponent = ELEMENT_TYPE_ICONS[elementType as keyof typeof ELEMENT_TYPE_ICONS] || Settings;
    return IconComponent;
  };

  const renderCategoryTree = (category: Category, depth: number = 0) => {
    const isExpanded = expandedCategories.has(category.id);
    const hasChildren = category.subcategories.length > 0 || category.elements.length > 0;
    const isSelected = selectedCategory === category.id;

    return (
      <div key={category.id} className="select-none">
        <div
          className={`flex items-center gap-2 p-2 rounded cursor-pointer hover:bg-gray-100 ${
            isSelected ? 'bg-blue-100 border border-blue-300' : ''
          }`}
          style={{ paddingLeft: `${depth * 20 + 8}px` }}
          onClick={() => {
            setSelectedCategory(category.id);
            if (hasChildren) {
              toggleCategory(category.id);
            }
          }}
          onDragOver={handleDragOver}
          onDrop={(e) => handleDrop(e, category.id)}
        >
          {hasChildren ? (
            isExpanded ? (
              <ChevronDown className="h-4 w-4 text-gray-500" />
            ) : (
              <ChevronRight className="h-4 w-4 text-gray-500" />
            )
          ) : (
            <div className="w-4" />
          )}
          
          {isExpanded ? (
            <FolderOpen className="h-4 w-4 text-gray-600" />
          ) : (
            <Folder className="h-4 w-4 text-gray-600" />
          )}
          
          <span className="font-medium text-gray-900">{category.name}</span>
          
          <Badge variant="outline" className="ml-auto">
            {category.element_count}
          </Badge>
          
          <div className="flex items-center gap-1">
            <Button
              variant="ghost"
              size="sm"
              onClick={(e) => {
                e.stopPropagation();
                setEditingCategory(category);
              }}
            >
              <Edit className="h-3 w-3" />
            </Button>
            <Button
              variant="ghost"
              size="sm"
              onClick={(e) => {
                e.stopPropagation();
                if (confirm('Are you sure you want to delete this category?')) {
                  onDeleteCategory(category.id);
                }
              }}
            >
              <Trash2 className="h-3 w-3" />
            </Button>
          </div>
        </div>
        
        {isExpanded && (
          <div>
            {/* Subcategories */}
            {category.subcategories.map(subcategory => 
              renderCategoryTree(subcategory, depth + 1)
            )}
            
            {/* Elements */}
            {category.elements.map(element => {
              const ElementIcon = getElementIcon(element.element_type);
              return (
                <div
                  key={element.id}
                  draggable
                  onDragStart={() => handleDragStart(element.id)}
                  className={`flex items-center gap-2 p-2 rounded cursor-pointer hover:bg-gray-50 ${
                    selectedElement?.id === element.id ? 'bg-blue-50 border border-blue-200' : ''
                  }`}
                  style={{ paddingLeft: `${(depth + 1) * 20 + 8}px` }}
                  onClick={() => setSelectedElement(element)}
                >
                  <div className="w-4" />
                  <ElementIcon className="h-4 w-4 text-gray-500" />
                  <span className="text-gray-800">{element.name}</span>
                  
                  <div className="flex items-center gap-1 ml-auto">
                    {element.visibility === 'never' && (
                      <EyeOff className="h-3 w-3 text-gray-400" />
                    )}
                    {element.series_shared && (
                      <Badge variant="outline" className="text-xs">
                        Shared
                      </Badge>
                    )}
                    
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={(e) => {
                        e.stopPropagation();
                        setEditingElement(element);
                      }}
                    >
                      <Edit className="h-3 w-3" />
                    </Button>
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={(e) => {
                        e.stopPropagation();
                        if (confirm('Are you sure you want to delete this element?')) {
                          onDeleteElement(element.id);
                        }
                      }}
                    >
                      <Trash2 className="h-3 w-3" />
                    </Button>
                  </div>
                </div>
              );
            })}
          </div>
        )}
      </div>
    );
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold text-gray-900">Hierarchical Worldbuilding</h2>
          <p className="text-gray-600">
            Organize your world elements into categories and hierarchies
          </p>
        </div>
        <div className="flex items-center gap-2">
          <Dialog open={showCreateCategoryDialog} onOpenChange={setShowCreateCategoryDialog}>
            <DialogTrigger asChild>
              <Button variant="outline">
                <FolderPlus className="h-4 w-4 mr-1" />
                Add Category
              </Button>
            </DialogTrigger>
            <DialogContent>
              <DialogHeader>
                <DialogTitle>Create Category</DialogTitle>
              </DialogHeader>
              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Name *
                  </label>
                  <Input
                    value={newCategory.name || ''}
                    onChange={(e) => setNewCategory({ ...newCategory, name: e.target.value })}
                    placeholder="Category name"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Description
                  </label>
                  <Textarea
                    value={newCategory.description || ''}
                    onChange={(e) => setNewCategory({ ...newCategory, description: e.target.value })}
                    placeholder="Describe this category"
                    rows={3}
                  />
                </div>
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Parent Category
                    </label>
                    <Select 
                      value={newCategory.parent_id || ''} 
                      onValueChange={(value) => setNewCategory({ ...newCategory, parent_id: value || undefined })}
                    >
                      <SelectTrigger>
                        <SelectValue placeholder="None (Root level)" />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="">None (Root level)</SelectItem>
                        {categories.map(category => (
                          <SelectItem key={category.id} value={category.id}>
                            {category.name}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Color
                    </label>
                    <Select 
                      value={newCategory.color || 'bg-blue-500'} 
                      onValueChange={(value) => setNewCategory({ ...newCategory, color: value })}
                    >
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        {CATEGORY_COLORS.map(color => (
                          <SelectItem key={color} value={color}>
                            <div className="flex items-center gap-2">
                              <div className={`w-4 h-4 rounded ${color}`} />
                              {color.replace('bg-', '').replace('-500', '')}
                            </div>
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </div>
                </div>
                <div className="flex justify-end gap-2">
                  <Button variant="outline" onClick={() => setShowCreateCategoryDialog(false)}>
                    Cancel
                  </Button>
                  <Button onClick={handleCreateCategory}>
                    Create Category
                  </Button>
                </div>
              </div>
            </DialogContent>
          </Dialog>
          
          <Dialog open={showCreateElementDialog} onOpenChange={setShowCreateElementDialog}>
            <DialogTrigger asChild>
              <Button>
                <Plus className="h-4 w-4 mr-1" />
                Add Element
              </Button>
            </DialogTrigger>
            <DialogContent className="max-w-2xl">
              <DialogHeader>
                <DialogTitle>Create World Element</DialogTitle>
              </DialogHeader>
              <div className="space-y-4">
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Name *
                    </label>
                    <Input
                      value={newElement.name || ''}
                      onChange={(e) => setNewElement({ ...newElement, name: e.target.value })}
                      placeholder="Element name"
                    />
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Type
                    </label>
                    <Select 
                      value={newElement.element_type || 'location'} 
                      onValueChange={(value) => setNewElement({ ...newElement, element_type: value })}
                    >
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        {Object.keys(ELEMENT_TYPE_ICONS).map(type => (
                          <SelectItem key={type} value={type}>
                            {type.charAt(0).toUpperCase() + type.slice(1)}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </div>
                </div>
                
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Description
                  </label>
                  <Textarea
                    value={newElement.description || ''}
                    onChange={(e) => setNewElement({ ...newElement, description: e.target.value })}
                    placeholder="Brief description"
                    rows={2}
                  />
                </div>
                
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Details
                  </label>
                  <Textarea
                    value={newElement.details || ''}
                    onChange={(e) => setNewElement({ ...newElement, details: e.target.value })}
                    placeholder="Detailed information"
                    rows={4}
                  />
                </div>
                
                <div className="grid grid-cols-3 gap-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Category
                    </label>
                    <Select 
                      value={newElement.category_id || selectedCategory || ''} 
                      onValueChange={(value) => setNewElement({ ...newElement, category_id: value || undefined })}
                    >
                      <SelectTrigger>
                        <SelectValue placeholder="Select category" />
                      </SelectTrigger>
                      <SelectContent>
                        {categories.map(category => (
                          <SelectItem key={category.id} value={category.id}>
                            {category.name}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Visibility
                    </label>
                    <Select 
                      value={newElement.visibility || 'always'} 
                      onValueChange={(value) => setNewElement({ ...newElement, visibility: value as any })}
                    >
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="always">Always Visible</SelectItem>
                        <SelectItem value="chapter">Chapter Context</SelectItem>
                        <SelectItem value="never">Hidden</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                  <div className="flex items-center space-x-2 pt-6">
                    <input
                      type="checkbox"
                      id="series-shared"
                      checked={newElement.series_shared || false}
                      onChange={(e) => setNewElement({ ...newElement, series_shared: e.target.checked })}
                      className="rounded border-gray-300"
                    />
                    <label htmlFor="series-shared" className="text-sm font-medium text-gray-700">
                      Share in Series
                    </label>
                  </div>
                </div>
                
                <div className="flex justify-end gap-2">
                  <Button variant="outline" onClick={() => setShowCreateElementDialog(false)}>
                    Cancel
                  </Button>
                  <Button onClick={handleCreateElement}>
                    Create Element
                  </Button>
                </div>
              </div>
            </DialogContent>
          </Dialog>
        </div>
      </div>

      {/* Search and Filters */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Search className="h-5 w-5" />
            Search & Filter
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
            <Input
              placeholder="Search elements..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
            />
            <Select value={filterType} onValueChange={setFilterType}>
              <SelectTrigger>
                <SelectValue placeholder="All Types" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="">All Types</SelectItem>
                {Object.keys(ELEMENT_TYPE_ICONS).map(type => (
                  <SelectItem key={type} value={type}>
                    {type.charAt(0).toUpperCase() + type.slice(1)}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            <Select value={filterVisibility} onValueChange={setFilterVisibility}>
              <SelectTrigger>
                <SelectValue placeholder="All Visibility" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="">All Visibility</SelectItem>
                <SelectItem value="always">Always Visible</SelectItem>
                <SelectItem value="chapter">Chapter Context</SelectItem>
                <SelectItem value="never">Hidden</SelectItem>
              </SelectContent>
            </Select>
            <div className="flex space-x-1 bg-gray-100 p-1 rounded-lg">
              {[
                { id: 'tree', label: 'Tree', icon: Tree },
                { id: 'grid', label: 'Grid', icon: Globe },
                { id: 'list', label: 'List', icon: Settings }
              ].map(viewOption => {
                const Icon = viewOption.icon;
                return (
                  <button
                    key={viewOption.id}
                    onClick={() => setView(viewOption.id as any)}
                    className={`flex items-center gap-1 px-3 py-1 rounded text-sm font-medium transition-colors ${
                      view === viewOption.id
                        ? 'bg-white text-gray-900 shadow-sm'
                        : 'text-gray-600 hover:text-gray-900'
                    }`}
                  >
                    <Icon className="h-3 w-3" />
                    {viewOption.label}
                  </button>
                );
              })}
            </div>
          </div>
        </CardContent>
      </Card>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Category Tree */}
        <div className="lg:col-span-1">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Folder className="h-5 w-5" />
                Categories
              </CardTitle>
            </CardHeader>
            <CardContent>
              {view === 'tree' && (
                <div className="space-y-1">
                  {hierarchicalCategories.length === 0 ? (
                    <div className="text-center py-8">
                      <Folder className="h-12 w-12 text-gray-400 mx-auto mb-4" />
                      <h3 className="text-lg font-medium text-gray-900 mb-2">
                        No Categories
                      </h3>
                      <p className="text-gray-600 mb-4">
                        Create your first category to organize world elements.
                      </p>
                      <Button onClick={() => setShowCreateCategoryDialog(true)}>
                        <FolderPlus className="h-4 w-4 mr-1" />
                        Create Category
                      </Button>
                    </div>
                  ) : (
                    hierarchicalCategories.map(category => renderCategoryTree(category))
                  )}
                </div>
              )}
            </CardContent>
          </Card>
        </div>

        {/* Element Details */}
        <div className="lg:col-span-2">
          {selectedElement ? (
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  {React.createElement(getElementIcon(selectedElement.element_type), { className: 'h-5 w-5' })}
                  {selectedElement.name}
                </CardTitle>
              </CardHeader>
              <CardContent>
                <div className="space-y-4">
                  <div>
                    <h4 className="font-medium text-gray-900 mb-2">Description</h4>
                    <p className="text-gray-600">{selectedElement.description}</p>
                  </div>
                  
                  {selectedElement.details && (
                    <div>
                      <h4 className="font-medium text-gray-900 mb-2">Details</h4>
                      <p className="text-gray-600 whitespace-pre-wrap">{selectedElement.details}</p>
                    </div>
                  )}
                  
                  <div className="grid grid-cols-2 gap-4">
                    <div>
                      <h4 className="font-medium text-gray-900 mb-2">Type</h4>
                      <Badge variant="outline">
                        {selectedElement.element_type.charAt(0).toUpperCase() + selectedElement.element_type.slice(1)}
                      </Badge>
                    </div>
                    <div>
                      <h4 className="font-medium text-gray-900 mb-2">Visibility</h4>
                      <Badge variant={selectedElement.visibility === 'always' ? 'default' : 'secondary'}>
                        {selectedElement.visibility === 'always' ? (
                          <><Eye className="h-3 w-3 mr-1" /> Always</>
                        ) : selectedElement.visibility === 'chapter' ? (
                          <><Eye className="h-3 w-3 mr-1" /> Chapter</>
                        ) : (
                          <><EyeOff className="h-3 w-3 mr-1" /> Hidden</>
                        )}
                      </Badge>
                    </div>
                  </div>
                  
                  {selectedElement.properties.length > 0 && (
                    <div>
                      <h4 className="font-medium text-gray-900 mb-2">Properties</h4>
                      <div className="space-y-2">
                        {selectedElement.properties.map((property, index) => (
                          <div key={index} className="flex justify-between p-2 bg-gray-50 rounded">
                            <span className="font-medium text-gray-700">{property.property_name}:</span>
                            <span className="text-gray-600">{property.property_value}</span>
                          </div>
                        ))}
                      </div>
                    </div>
                  )}
                  
                  {selectedElement.tags.length > 0 && (
                    <div>
                      <h4 className="font-medium text-gray-900 mb-2">Tags</h4>
                      <div className="flex flex-wrap gap-1">
                        {selectedElement.tags.map(tag => (
                          <Badge key={tag} variant="secondary">
                            {tag}
                          </Badge>
                        ))}
                      </div>
                    </div>
                  )}
                  
                  {selectedElement.relationships.length > 0 && (
                    <div>
                      <h4 className="font-medium text-gray-900 mb-2">Relationships</h4>
                      <div className="space-y-2">
                        {selectedElement.relationships.map((relationship, index) => (
                          <div key={index} className="p-2 border rounded">
                            <div className="font-medium text-gray-700">{relationship.relationship_type}</div>
                            <div className="text-sm text-gray-600">{relationship.description}</div>
                          </div>
                        ))}
                      </div>
                    </div>
                  )}
                  
                  <div className="flex gap-2 pt-4 border-t">
                    <Button
                      variant="outline"
                      onClick={() => setEditingElement(selectedElement)}
                    >
                      <Edit className="h-4 w-4 mr-1" />
                      Edit
                    </Button>
                    <Button
                      variant="destructive"
                      onClick={() => {
                        if (confirm('Are you sure you want to delete this element?')) {
                          onDeleteElement(selectedElement.id);
                          setSelectedElement(null);
                        }
                      }}
                    >
                      <Trash2 className="h-4 w-4 mr-1" />
                      Delete
                    </Button>
                  </div>
                </div>
              </CardContent>
            </Card>
          ) : (
            <Card>
              <CardContent className="p-12 text-center">
                <Globe className="h-12 w-12 text-gray-400 mx-auto mb-4" />
                <h3 className="text-lg font-medium text-gray-900 mb-2">
                  Select an Element
                </h3>
                <p className="text-gray-600">
                  Choose an element from the category tree to view its details.
                </p>
              </CardContent>
            </Card>
          )}
        </div>
      </div>
    </div>
  );
};

export default HierarchicalWorldbuilding;