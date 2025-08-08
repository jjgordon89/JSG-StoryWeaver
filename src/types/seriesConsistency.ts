// Series consistency checking types

export interface ConsistencyConflict {
  conflict_type: ConflictType;
  severity: ConflictSeverity;
  description: string;
  affected_projects: string[];
  details: Record<string, any>;
}

export interface SeriesConsistencyReport {
  series_id: string;
  consistency_score: number; // 0.0 to 1.0
  total_conflicts: number;
  conflicts: ConsistencyConflict[];
  character_conflicts: number;
  world_element_conflicts: number;
  story_bible_conflicts: number;
  generated_at: string; // ISO timestamp
}

export interface CharacterConsistencyData {
  character_name: string;
  project_id: string;
  traits: Record<string, string>;
}

export interface WorldElementConsistencyData {
  element_name: string;
  element_type: string;
  project_id: string;
  content: string;
}

export enum ConflictType {
  CharacterInconsistency = 'CharacterInconsistency',
  WorldElementInconsistency = 'WorldElementInconsistency',
  StoryBibleMismatch = 'StoryBibleMismatch',
  TimelineConflict = 'TimelineConflict',
  PlotInconsistency = 'PlotInconsistency'
}

export enum ConflictSeverity {
  Critical = 'Critical',
  High = 'High',
  Medium = 'Medium',
  Low = 'Low'
}

// API response types
export interface SeriesConsistencyStatus {
  consistency_score: number;
  conflict_count: number;
}

export interface BatchConsistencyResult {
  series_id: string;
  consistency_score: number;
  conflict_count: number;
}

// UI component props
export interface ConsistencyReportProps {
  seriesId: string;
  onRefresh?: () => void;
}

export interface ConflictListProps {
  conflicts: ConsistencyConflict[];
  onResolve?: (conflict: ConsistencyConflict) => void;
}

export interface ConsistencyScoreProps {
  score: number;
  conflictCount: number;
  size?: 'small' | 'medium' | 'large';
}

// Filter and sorting options
export interface ConsistencyFilters {
  severity?: ConflictSeverity[];
  conflictType?: ConflictType[];
  projectIds?: string[];
}

export interface ConsistencySortOptions {
  field: 'severity' | 'type' | 'project' | 'created';
  direction: 'asc' | 'desc';
}

// Utility functions type definitions
export type ConflictResolver = (conflict: ConsistencyConflict) => Promise<boolean>;
export type ConsistencyAnalyzer = (seriesId: string) => Promise<SeriesConsistencyReport>;