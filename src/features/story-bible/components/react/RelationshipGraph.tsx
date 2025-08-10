import React, { useEffect, useRef, useState } from 'react';
import { Button } from '../../../../ui/components/common';
import { Card, CardContent, CardHeader, CardTitle } from '../../../../ui/components/common';
import { ZoomIn, ZoomOut, RotateCcw, Download } from 'lucide-react';

interface Character {
  id: string;
  name: string;
  description?: string;
}

interface Relationship {
  id: string;
  fromCharacterId: string;
  toCharacterId: string;
  relationshipType: string;
  description: string;
  strength: number; // 1-10 scale
  isPublic: boolean;
}

interface RelationshipGraphProps {
  characters: Character[];
  relationships: Relationship[];
  onNodeClick?: (characterId: string) => void;
  onRelationshipClick?: (relationship: Relationship) => void;
}

interface Node {
  id: string;
  name: string;
  x: number;
  y: number;
  vx: number;
  vy: number;
  radius: number;
  color: string;
}

interface Edge {
  source: Node;
  target: Node;
  relationship: Relationship;
  color: string;
  width: number;
}

const RelationshipGraph: React.FC<RelationshipGraphProps> = ({
  characters,
  relationships,
  onNodeClick,
  onRelationshipClick
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [nodes, setNodes] = useState<Node[]>([]);
  const [edges, setEdges] = useState<Edge[]>([]);
  const [scale, setScale] = useState(1);
  const [offsetX, setOffsetX] = useState(0);
  const [offsetY, setOffsetY] = useState(0);
  const [isDragging, setIsDragging] = useState(false);
  const [dragNode, setDragNode] = useState<Node | null>(null);
  const [lastMousePos, setLastMousePos] = useState({ x: 0, y: 0 });
  const animationRef = useRef<number>();

  // Relationship type colors
  const relationshipColors: Record<string, string> = {
    family: '#8B5CF6',
    romantic: '#EC4899',
    friend: '#10B981',
    enemy: '#EF4444',
    ally: '#3B82F6',
    mentor: '#F59E0B',
    rival: '#F97316',
    colleague: '#6B7280',
    acquaintance: '#9CA3AF',
    other: '#64748B'
  };

  // Initialize nodes and edges
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const centerX = canvas.width / 2;
    const centerY = canvas.height / 2;
    const radius = Math.min(canvas.width, canvas.height) * 0.3;

    // Create nodes
    const newNodes: Node[] = characters.map((character, index) => {
      const angle = (index / characters.length) * 2 * Math.PI;
      return {
        id: character.id,
        name: character.name,
        x: centerX + Math.cos(angle) * radius,
        y: centerY + Math.sin(angle) * radius,
        vx: 0,
        vy: 0,
        radius: 20,
        color: '#3B82F6'
      };
    });

    // Create edges
    const newEdges: Edge[] = relationships.map(relationship => {
      const sourceNode = newNodes.find(n => n.id === relationship.fromCharacterId);
      const targetNode = newNodes.find(n => n.id === relationship.toCharacterId);
      
      if (!sourceNode || !targetNode) {
        return null;
      }

      return {
        source: sourceNode,
        target: targetNode,
        relationship,
        color: relationshipColors[relationship.relationshipType] || '#64748B',
        width: Math.max(1, relationship.strength / 2)
      };
    }).filter(Boolean) as Edge[];

    setNodes(newNodes);
    setEdges(newEdges);
  }, [characters, relationships]);

  // Physics simulation
  useEffect(() => {
    if (nodes.length === 0) return;

    const simulate = () => {
      const canvas = canvasRef.current;
      if (!canvas) return;

      // Apply forces
      nodes.forEach(node => {
        // Repulsion between nodes
        nodes.forEach(otherNode => {
          if (node === otherNode) return;
          
          const dx = node.x - otherNode.x;
          const dy = node.y - otherNode.y;
          const distance = Math.sqrt(dx * dx + dy * dy);
          
          if (distance > 0) {
            const force = 500 / (distance * distance);
            node.vx += (dx / distance) * force;
            node.vy += (dy / distance) * force;
          }
        });

        // Attraction along edges
        edges.forEach(edge => {
          if (edge.source === node) {
            const dx = edge.target.x - node.x;
            const dy = edge.target.y - node.y;
            const distance = Math.sqrt(dx * dx + dy * dy);
            const force = distance * 0.01;
            
            node.vx += (dx / distance) * force;
            node.vy += (dy / distance) * force;
          }
          if (edge.target === node) {
            const dx = edge.source.x - node.x;
            const dy = edge.source.y - node.y;
            const distance = Math.sqrt(dx * dx + dy * dy);
            const force = distance * 0.01;
            
            node.vx += (dx / distance) * force;
            node.vy += (dy / distance) * force;
          }
        });

        // Center attraction
        const centerX = canvas.width / 2;
        const centerY = canvas.height / 2;
        const dx = centerX - node.x;
        const dy = centerY - node.y;
        node.vx += dx * 0.001;
        node.vy += dy * 0.001;

        // Apply velocity with damping
        node.vx *= 0.9;
        node.vy *= 0.9;
        node.x += node.vx;
        node.y += node.vy;

        // Keep nodes within bounds
        node.x = Math.max(node.radius, Math.min(canvas.width - node.radius, node.x));
        node.y = Math.max(node.radius, Math.min(canvas.height - node.radius, node.y));
      });

      draw();
      animationRef.current = requestAnimationFrame(simulate);
    };

    animationRef.current = requestAnimationFrame(simulate);

    return () => {
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current);
      }
    };
  }, [nodes, edges]);

  const draw = () => {
    const canvas = canvasRef.current;
    const ctx = canvas?.getContext('2d');
    if (!canvas || !ctx) return;

    // Clear canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    
    // Apply transformations
    ctx.save();
    ctx.translate(offsetX, offsetY);
    ctx.scale(scale, scale);

    // Draw edges
    edges.forEach(edge => {
      ctx.beginPath();
      ctx.moveTo(edge.source.x, edge.source.y);
      ctx.lineTo(edge.target.x, edge.target.y);
      ctx.strokeStyle = edge.color;
      ctx.lineWidth = edge.width;
      ctx.stroke();

      // Draw relationship label
      const midX = (edge.source.x + edge.target.x) / 2;
      const midY = (edge.source.y + edge.target.y) / 2;
      
      ctx.fillStyle = '#374151';
      ctx.font = '12px sans-serif';
      ctx.textAlign = 'center';
      ctx.fillText(edge.relationship.relationshipType, midX, midY - 5);
    });

    // Draw nodes
    nodes.forEach(node => {
      // Node circle
      ctx.beginPath();
      ctx.arc(node.x, node.y, node.radius, 0, 2 * Math.PI);
      ctx.fillStyle = node.color;
      ctx.fill();
      ctx.strokeStyle = '#1F2937';
      ctx.lineWidth = 2;
      ctx.stroke();

      // Node label
      ctx.fillStyle = '#FFFFFF';
      ctx.font = 'bold 12px sans-serif';
      ctx.textAlign = 'center';
      ctx.fillText(node.name, node.x, node.y + 4);
    });

    ctx.restore();
  };

  const handleMouseDown = (e: React.MouseEvent) => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    const mouseX = (e.clientX - rect.left - offsetX) / scale;
    const mouseY = (e.clientY - rect.top - offsetY) / scale;

    // Check if clicking on a node
    const clickedNode = nodes.find(node => {
      const dx = mouseX - node.x;
      const dy = mouseY - node.y;
      return Math.sqrt(dx * dx + dy * dy) <= node.radius;
    });

    if (clickedNode) {
      setDragNode(clickedNode);
      if (onNodeClick) {
        onNodeClick(clickedNode.id);
      }
    } else {
      setIsDragging(true);
    }

    setLastMousePos({ x: e.clientX, y: e.clientY });
  };

  const handleMouseMove = (e: React.MouseEvent) => {
    if (dragNode) {
      const canvas = canvasRef.current;
      if (!canvas) return;

      const rect = canvas.getBoundingClientRect();
      const mouseX = (e.clientX - rect.left - offsetX) / scale;
      const mouseY = (e.clientY - rect.top - offsetY) / scale;

      dragNode.x = mouseX;
      dragNode.y = mouseY;
      dragNode.vx = 0;
      dragNode.vy = 0;
    } else if (isDragging) {
      const dx = e.clientX - lastMousePos.x;
      const dy = e.clientY - lastMousePos.y;
      
      setOffsetX(prev => prev + dx);
      setOffsetY(prev => prev + dy);
      setLastMousePos({ x: e.clientX, y: e.clientY });
    }
  };

  const handleMouseUp = () => {
    setIsDragging(false);
    setDragNode(null);
  };

  const handleWheel = (e: React.WheelEvent) => {
    e.preventDefault();
    const delta = e.deltaY > 0 ? 0.9 : 1.1;
    setScale(prev => Math.max(0.1, Math.min(3, prev * delta)));
  };

  const resetView = () => {
    setScale(1);
    setOffsetX(0);
    setOffsetY(0);
  };

  const zoomIn = () => {
    setScale(prev => Math.min(3, prev * 1.2));
  };

  const zoomOut = () => {
    setScale(prev => Math.max(0.1, prev / 1.2));
  };

  const exportGraph = () => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const link = document.createElement('a');
    link.download = 'character-relationships.png';
    link.href = canvas.toDataURL();
    link.click();
  };

  return (
    <Card className="w-full h-full">
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle>Character Relationship Graph</CardTitle>
          <div className="flex items-center gap-2">
            <Button variant="outline" size="sm" onClick={zoomOut}>
              <ZoomOut className="h-4 w-4" />
            </Button>
            <Button variant="outline" size="sm" onClick={zoomIn}>
              <ZoomIn className="h-4 w-4" />
            </Button>
            <Button variant="outline" size="sm" onClick={resetView}>
              <RotateCcw className="h-4 w-4" />
            </Button>
            <Button variant="outline" size="sm" onClick={exportGraph}>
              <Download className="h-4 w-4" />
            </Button>
          </div>
        </div>
      </CardHeader>
      <CardContent>
        <div className="relative">
          <canvas
            ref={canvasRef}
            width={800}
            height={600}
            className="border border-gray-200 rounded cursor-move"
            onMouseDown={handleMouseDown}
            onMouseMove={handleMouseMove}
            onMouseUp={handleMouseUp}
            onMouseLeave={handleMouseUp}
            onWheel={handleWheel}
          />
          
          {/* Legend */}
          <div className="absolute top-4 right-4 bg-white p-3 rounded-lg shadow-lg border">
            <h4 className="font-semibold text-sm mb-2">Relationship Types</h4>
            <div className="space-y-1 text-xs">
              {Object.entries(relationshipColors).map(([type, color]) => (
                <div key={type} className="flex items-center gap-2">
                  <div 
                    className="w-3 h-0.5" 
                    style={{ backgroundColor: color }}
                  />
                  <span className="capitalize">{type}</span>
                </div>
              ))}
            </div>
          </div>
        </div>
        
        <div className="mt-4 text-sm text-gray-600">
          <p>• Click and drag nodes to reposition them</p>
          <p>• Click and drag the background to pan</p>
          <p>• Use mouse wheel to zoom in/out</p>
          <p>• Click on nodes to select characters</p>
        </div>
      </CardContent>
    </Card>
  );
};

export default RelationshipGraph;