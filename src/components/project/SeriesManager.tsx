import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface Series {
  id: string;
  name: string;
  description: string;
  project_count: number;
  created_at: string;
}

interface Project {
  id: string;
  name: string;
  description: string;
  series_id: string | null;
}

interface SeriesManagerProps {
  onSeriesSelect?: (seriesId: string) => void;
  onProjectSelect?: (projectId: string) => void;
}

const SeriesManager: React.FC<SeriesManagerProps> = ({ onSeriesSelect, onProjectSelect }) => {
  const [series, setSeries] = useState<Series[]>([]);
  const [projects, setProjects] = useState<Project[]>([]);
  const [selectedSeries, setSelectedSeries] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [newSeriesName, setNewSeriesName] = useState('');
  const [newSeriesDescription, setNewSeriesDescription] = useState('');
  const [showNewSeriesForm, setShowNewSeriesForm] = useState(false);

  // Fetch series data
  useEffect(() => {
    const fetchSeries = async () => {
      setLoading(true);
      try {
        // Fetch series with project counts
        const seriesResponse = await invoke<{ data: Series[] }>('get_series_with_counts');
        if (!seriesResponse.data) {
          throw new Error('Failed to fetch series data');
        }
        setSeries(seriesResponse.data);
        
        // Fetch all projects
        const projectsResponse = await invoke<{ data: Project[] }>('get_projects');
        if (!projectsResponse.data) {
          throw new Error('Failed to fetch projects');
        }
        setProjects(projectsResponse.data);
      } catch (err) {
        console.error('Error fetching series:', err);
        setError('Failed to load series data');
      } finally {
        setLoading(false);
      }
    };
    
    fetchSeries();
  }, []);

  // Handle series selection
  const handleSeriesSelect = (seriesId: string) => {
    setSelectedSeries(seriesId === selectedSeries ? null : seriesId);
    if (onSeriesSelect && seriesId !== selectedSeries) {
      onSeriesSelect(seriesId);
    }
  };

  // Create a new series
  const handleCreateSeries = async () => {
    if (!newSeriesName.trim()) return;
    
    try {
      // Call the backend API to create a series
      const response = await invoke<{ data: Series }>('create_series', {
        request: {
          name: newSeriesName,
          description: newSeriesDescription,
          folder_id: null // Could be set if needed
        }
      });
      
      if (!response.data) {
        throw new Error('Failed to create series');
      }
      
      // Refresh series data
      const seriesResponse = await invoke<{ data: Series[] }>('get_series_with_counts');
      if (seriesResponse.data) {
        setSeries(seriesResponse.data);
      }
      
      setNewSeriesName('');
      setNewSeriesDescription('');
      setShowNewSeriesForm(false);
    } catch (err) {
      console.error('Error creating series:', err);
      setError('Failed to create series');
    }
  };

  // Add project to series
  const handleAddProjectToSeries = async (projectId: string, seriesId: string) => {
    try {
      // Call the backend API to add project to series
      await invoke('add_project_to_series', { seriesId, projectId });
      
      // Update local state
      setProjects(projects.map(project => 
        project.id === projectId ? { ...project, series_id: seriesId } : project
      ));
      
      // Refresh series data to update counts
      const seriesResponse = await invoke<{ data: Series[] }>('get_series_with_counts');
      if (seriesResponse.data) {
        setSeries(seriesResponse.data);
      }
    } catch (err) {
      console.error('Error adding project to series:', err);
      setError('Failed to add project to series');
    }
  };

  // Remove project from series
  const handleRemoveProjectFromSeries = async (projectId: string) => {
    try {
      // Call the backend API to remove project from series
      await invoke('remove_project_from_series', { projectId });
      
      // Update local state
      setProjects(projects.map(project => 
        project.id === projectId ? { ...project, series_id: null } : project
      ));
      
      // Refresh series data to update counts
      const seriesResponse = await invoke<{ data: Series[] }>('get_series_with_counts');
      if (seriesResponse.data) {
        setSeries(seriesResponse.data);
      }
    } catch (err) {
      console.error('Error removing project from series:', err);
      setError('Failed to remove project from series');
    }
  };

  if (loading) {
    return <div className="p-4">Loading series data...</div>;
  }

  if (error) {
    return <div className="p-4 text-red-500">{error}</div>;
  }

  // Get projects for the selected series
  const seriesProjects = selectedSeries 
    ? projects.filter(project => project.series_id === selectedSeries)
    : [];
  
  // Get projects not in any series
  const standaloneProjects = projects.filter(project => project.series_id === null);

  return (
    <div className="series-manager">
      <div className="flex justify-between items-center mb-4">
        <h3 className="text-lg font-semibold">Series</h3>
        <button 
          className="text-sm bg-blue-500 hover:bg-blue-700 text-white px-2 py-1 rounded"
          onClick={() => setShowNewSeriesForm(!showNewSeriesForm)}
        >
          {showNewSeriesForm ? 'Cancel' : 'New Series'}
        </button>
      </div>
      
      {/* New Series Form */}
      {showNewSeriesForm && (
        <div className="mb-4 p-3 border rounded-md bg-gray-50 dark:bg-gray-800">
          <h4 className="text-sm font-medium mb-2">Create New Series</h4>
          <input
            type="text"
            placeholder="Series Name"
            className="w-full p-2 mb-2 border rounded dark:bg-gray-700"
            value={newSeriesName}
            onChange={(e) => setNewSeriesName(e.target.value)}
          />
          <textarea
            placeholder="Description"
            className="w-full p-2 mb-2 border rounded dark:bg-gray-700"
            rows={3}
            value={newSeriesDescription}
            onChange={(e) => setNewSeriesDescription(e.target.value)}
          />
          <button
            className="w-full bg-green-500 hover:bg-green-700 text-white py-1 px-2 rounded"
            onClick={handleCreateSeries}
          >
            Create Series
          </button>
        </div>
      )}
      
      {/* Series List */}
      <div className="space-y-2 mb-6">
        {series.length === 0 ? (
          <div className="p-4 text-gray-500 bg-white dark:bg-gray-800 rounded-md">
            No series yet. Create your first series to organize related projects.
          </div>
        ) : (
          series.map((s) => (
            <div 
              key={s.id}
              className={`p-3 rounded-md cursor-pointer transition-colors
                ${selectedSeries === s.id 
                  ? 'bg-blue-100 dark:bg-blue-900 border-l-4 border-blue-500' 
                  : 'bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-750'}`}
              onClick={() => handleSeriesSelect(s.id)}
            >
              <div className="flex justify-between items-center">
                <h4 className="font-medium">{s.name}</h4>
                <span className="text-xs bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 px-2 py-1 rounded">
                  {s.project_count} projects
                </span>
              </div>
              <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">{s.description}</p>
            </div>
          ))
        )}
      </div>
      
      {/* Projects in Selected Series */}
      {selectedSeries && (
        <div>
          <h4 className="font-medium mb-2">Projects in this Series</h4>
          {seriesProjects.length === 0 ? (
            <p className="text-sm text-gray-500">No projects in this series yet.</p>
          ) : (
            <div className="space-y-1">
              {seriesProjects.map((project) => (
                <div 
                  key={project.id}
                  className="flex justify-between items-center p-2 bg-white dark:bg-gray-800 rounded hover:bg-gray-50 dark:hover:bg-gray-750"
                >
                  <div 
                    className="flex-grow cursor-pointer"
                    onClick={() => onProjectSelect && onProjectSelect(project.id)}
                  >
                    <div className="font-medium">{project.name}</div>
                    <div className="text-xs text-gray-500">{project.description}</div>
                  </div>
                  <button
                    className="text-xs text-red-500 hover:text-red-700"
                    onClick={(e) => {
                      e.stopPropagation();
                      handleRemoveProjectFromSeries(project.id);
                    }}
                  >
                    Remove
                  </button>
                </div>
              ))}
            </div>
          )}
          
          {/* Add Projects to Series */}
          {standaloneProjects.length > 0 && (
            <div className="mt-4">
              <h4 className="font-medium mb-2">Add Projects to Series</h4>
              <div className="space-y-1">
                {standaloneProjects.map((project) => (
                  <div 
                    key={project.id}
                    className="flex justify-between items-center p-2 bg-white dark:bg-gray-800 rounded hover:bg-gray-50 dark:hover:bg-gray-750"
                  >
                    <div className="flex-grow">
                      <div className="font-medium">{project.name}</div>
                      <div className="text-xs text-gray-500">{project.description}</div>
                    </div>
                    <button
                      className="text-xs text-green-500 hover:text-green-700"
                      onClick={() => handleAddProjectToSeries(project.id, selectedSeries)}
                    >
                      Add
                    </button>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      )}
      
      {/* Standalone Projects (when no series is selected) */}
      {!selectedSeries && standaloneProjects.length > 0 && (
        <div>
          <h4 className="font-medium mb-2">Standalone Projects</h4>
          <div className="space-y-1">
            {standaloneProjects.map((project) => (
              <div 
                key={project.id}
                className="p-2 bg-white dark:bg-gray-800 rounded hover:bg-gray-50 dark:hover:bg-gray-750 cursor-pointer"
                onClick={() => onProjectSelect && onProjectSelect(project.id)}
              >
                <div className="font-medium">{project.name}</div>
                <div className="text-xs text-gray-500">{project.description}</div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

export default SeriesManager;
