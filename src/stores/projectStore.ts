import { create } from 'zustand';
import { invoke } from '../utils/tauriSafe';

interface Project {
  id: string;
  name: string;
  description: string | null;
  genre: string | null;
  target_word_count: number | null;
  current_word_count: number;
  status: string;
  created_at: string;
  updated_at: string;
}

interface Document {
  id: string;
  project_id: string;
  title: string;
  content: string;
  document_type: string;
  word_count: number;
  created_at: string;
  updated_at: string;
}

interface ProjectState {
  projects: Project[];
  currentProject: Project | null;
  currentDocument: Document | null;
  isLoading: boolean;
  error: string | null;
  
  // Actions
  loadProjects: () => Promise<void>;
  loadProject: (projectId: string) => Promise<void>;
  loadDocument: (documentId: string) => Promise<void>;
  createProject: (name: string, description?: string, genre?: string, targetWordCount?: number) => Promise<Project>;
  updateProject: (projectId: string, updates: Partial<Project>) => Promise<void>;
  deleteProject: (projectId: string) => Promise<void>;
  createDocument: (projectId: string, title: string, documentType?: string) => Promise<Document>;
  updateDocument: (documentId: string, updates: Partial<Document>) => Promise<void>;
  deleteDocument: (documentId: string) => Promise<void>;
  setCurrentProject: (project: Project | null) => void;
  setCurrentDocument: (document: Document | null) => void;
  insertTextAtCursor: (text: string) => void;
}

export const useProjectStore = create<ProjectState>((set, get) => ({
  projects: [],
  currentProject: null,
  currentDocument: null,
  isLoading: false,
  error: null,

  loadProjects: async () => {
    set({ isLoading: true, error: null });
    try {
      const projects = await invoke<Project[]>('get_projects');
      set({ projects, isLoading: false });
    } catch (error) {
      set({ error: error as string, isLoading: false });
    }
  },

  loadProject: async (projectId: string) => {
    set({ isLoading: true, error: null });
    try {
      const project = await invoke<Project>('get_project', { id: projectId });
      if (project) {
        set({ currentProject: project, isLoading: false });
      } else {
        set({ error: 'Project not found', isLoading: false });
      }
    } catch (error) {
      set({ error: error as string, isLoading: false });
    }
  },

  loadDocument: async (documentId: string) => {
    set({ isLoading: true, error: null });
    try {
      const document = await invoke<Document>('get_document', { documentId });
      if (document) {
        set({ currentDocument: document, isLoading: false });
      } else {
        set({ error: 'Document not found', isLoading: false });
      }
    } catch (error) {
      set({ error: error as string, isLoading: false });
    }
  },

  createProject: async (name: string, description?: string, genre?: string, targetWordCount?: number) => {
    set({ isLoading: true, error: null });
    try {
      const project = await invoke<Project>('create_project', { 
        name, 
        description, 
        genre, 
        target_word_count: targetWordCount 
      });
      set((state) => ({
        projects: [...state.projects, project],
        currentProject: project,
        isLoading: false,
      }));
      return project;
    } catch (error) {
      set({ error: error as string, isLoading: false });
      throw error;
    }
  },

  updateProject: async (projectId: string, updates: Partial<Project>) => {
    try {
      await invoke('update_project', { id: projectId, ...updates });
      
      // Update the projects array
      set((state) => ({
        projects: state.projects.map((proj) =>
          proj.id === projectId ? { ...proj, ...updates } : proj
        ),
      }));
      
      // Update current project if it matches
      const { currentProject } = get();
      if (currentProject && currentProject.id === projectId) {
        set({
          currentProject: { ...currentProject, ...updates },
        });
      }
    } catch (error) {
      set({ error: error as string });
    }
  },

  deleteProject: async (projectId: string) => {
    try {
      await invoke('delete_project', { id: projectId });
      
      set((state) => ({
        projects: state.projects.filter((proj) => proj.id !== projectId),
        currentProject: state.currentProject?.id === projectId ? null : state.currentProject,
      }));
    } catch (error) {
      set({ error: error as string });
    }
  },

  createDocument: async (projectId: string, title: string, documentType: string = 'chapter') => {
    set({ isLoading: true, error: null });
    try {
      const document = await invoke<Document>('create_document', { 
        projectId, 
        name: title,
        document_type: documentType 
      });
      set((state) => ({
        currentDocument: document,
        isLoading: false,
      }));
      return document;
    } catch (error) {
      set({ error: error as string, isLoading: false });
      throw error;
    }
  },

  updateDocument: async (documentId: string, updates: Partial<Document>) => {
    try {
      await invoke('update_document', { documentId, ...updates });
      
      // Update current document if it matches
      const { currentDocument } = get();
      if (currentDocument && currentDocument.id === documentId) {
        set({
          currentDocument: { ...currentDocument, ...updates },
        });
      }
    } catch (error) {
      set({ error: error as string });
    }
  },

  deleteDocument: async (documentId: string) => {
    try {
      await invoke('delete_document', { documentId });
      
      set((state) => ({
        currentDocument: state.currentDocument?.id === documentId ? null : state.currentDocument,
      }));
    } catch (error) {
      set({ error: error as string });
    }
  },

  setCurrentProject: (project: Project | null) => {
    set({ currentProject: project });
  },

  setCurrentDocument: (document: Document | null) => {
    set({ currentDocument: document });
  },

  insertTextAtCursor: (text: string) => {
    // This would integrate with the document editor to insert text at cursor position
    // For now, just update the current document content
    const { currentDocument } = get();
    if (currentDocument) {
      set({
        currentDocument: {
          ...currentDocument,
          content: currentDocument.content + text,
          updated_at: new Date().toISOString(),
        },
      });
    }
  },
}));
