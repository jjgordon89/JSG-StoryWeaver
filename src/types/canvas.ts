export interface Canvas {
  id: number;
  project_id: string;
  name: string;
  description?: string;
  canvas_data: string;
  template_type?: OutlineTemplateType;
  width: number;
  height: number;
  zoom_level: number;
  viewport_x: number;
  viewport_y: number;
  created_at: string;
  updated_at: string;
}

export interface CanvasElement {
  id: number;
  canvas_id: number;
  element_type: CanvasElementType;
  title: string;
  content: string;
  position_x: number;
  position_y: number;
  width: number;
  height: number;
  color: string;
  metadata: string;
  connections: string;
  order_index: number;
  created_at: string;
  updated_at: string;
}

export type CanvasElementType = 
  | 'plot_point'
  | 'character_arc'
  | 'scene'
  | 'chapter'
  | 'act'
  | 'note'
  | 'connection'
  | 'timeline_event'
  | 'theme'
  | 'conflict'
  | 'text_box'
  | 'sticky_note';

export type OutlineTemplateType =
  | 'heros_journey'
  | 'hollywood_beats'
  | 'story_circle'
  | 'romance_outline'
  | 'three_act'
  | 'save_the_cat'
  | 'snowflake'
  | 'seven_point'
  | 'custom';

export interface OutlineTemplate {
  id: number;
  name: string;
  description: string;
  template_type: OutlineTemplateType;
  structure_data: string;
  is_builtin: boolean;
  created_at: string;
}

export interface CanvasSnapshot {
  id: number;
  canvas_id: number;
  snapshot_name: string;
  canvas_data: string;
  created_at: string;
}

export interface CanvasCollaborationSession {
  id: number;
  canvas_id: number;
  session_token: string;
  host_user: string;
  participants: string;
  is_active: boolean;
  max_participants: number;
  current_participants: number;
  created_at: string;
  updated_at: string;
  expires_at?: string;
}

export interface CanvasOperation {
  id: string;
  canvas_id: number;
  operation_type: CanvasOperationType;
  element_id?: number;
  data: string;
  user_token: string;
  timestamp: number;
}

export type CanvasOperationType =
  | 'create_element'
  | 'update_element'
  | 'delete_element'
  | 'move_element'
  | 'resize_element'
  | 'create_connection'
  | 'delete_connection'
  | 'update_canvas';

export interface CanvasConnection {
  from_element_id: number;
  to_element_id: number;
  connection_type: ConnectionType;
  label?: string;
  style: ConnectionStyle;
}

export type ConnectionType =
  | 'sequence'
  | 'cause'
  | 'conflict'
  | 'resolution'
  | 'character'
  | 'theme'
  | 'custom';

export interface ConnectionStyle {
  line_type: LineType;
  color: string;
  thickness: number;
  arrow_type: ArrowType;
}

export type LineType = 'solid' | 'dashed' | 'dotted' | 'curved';
export type ArrowType = 'none' | 'single' | 'double' | 'diamond' | 'circle';

export type ExportFormat = 
  | 'story_bible'
  | 'outline'
  | 'json'
  | 'markdown'
  | 'image'
  | 'png'
  | 'svg'
  | 'pdf';

export interface CanvasExportResult {
  canvas_id: string;
  format: ExportFormat;
  data: string;
  file_size: number;
  exported_at: string;
}
