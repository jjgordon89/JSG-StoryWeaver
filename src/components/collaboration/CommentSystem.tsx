import React, { useState, useEffect, useRef } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { MessageCircle, Plus, X, Reply, Trash2, Edit3, Check, MoreHorizontal } from 'lucide-react';
import { Button } from '../../ui/components/common';
import { Avatar } from '../ui/avatar';

interface Comment {
  id: string;
  content: string;
  author: string;
  timestamp: Date;
  position: {
    line: number;
    column: number;
  };
  resolved: boolean;
  replies: Comment[];
}

interface CommentSystemProps {
  documentId: number;
  comments: Comment[];
  onAddComment: (content: string, position: { line: number; column: number }) => void;
  onReplyToComment: (commentId: string, content: string) => void;
  onResolveComment: (commentId: string) => void;
  onDeleteComment: (commentId: string) => void;
  onEditComment: (commentId: string, content: string) => void;
  currentUser: string;
}

interface CommentThreadProps {
  comment: Comment;
  onReply: (content: string) => void;
  onResolve: () => void;
  onDelete: () => void;
  onEdit: (content: string) => void;
  currentUser: string;
  isNested?: boolean;
}

const CommentThread: React.FC<CommentThreadProps> = ({
  comment,
  onReply,
  onResolve,
  onDelete,
  onEdit,
  currentUser,
  isNested = false
}) => {
  const [isReplying, setIsReplying] = useState(false);
  const [isEditing, setIsEditing] = useState(false);
  const [replyContent, setReplyContent] = useState('');
  const [editContent, setEditContent] = useState(comment.content);
  const [showActions, setShowActions] = useState(false);
  const replyInputRef = useRef<HTMLTextAreaElement>(null);
  const editInputRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    if (isReplying && replyInputRef.current) {
      replyInputRef.current.focus();
    }
    if (isEditing && editInputRef.current) {
      editInputRef.current.focus();
    }
  }, [isReplying, isEditing]);

  const handleReply = () => {
    if (replyContent.trim()) {
      onReply(replyContent.trim());
      setReplyContent('');
      setIsReplying(false);
    }
  };

  const handleEdit = () => {
    if (editContent.trim() && editContent !== comment.content) {
      onEdit(editContent.trim());
      setIsEditing(false);
    } else {
      setIsEditing(false);
    }
  };

  const formatTimestamp = (timestamp: Date) => {
    const now = new Date();
    const diff = now.getTime() - timestamp.getTime();
    const minutes = Math.floor(diff / 60000);
    const hours = Math.floor(diff / 3600000);
    const days = Math.floor(diff / 86400000);

    if (minutes < 1) return 'just now';
    if (minutes < 60) return `${minutes}m ago`;
    if (hours < 24) return `${hours}h ago`;
    if (days < 7) return `${days}d ago`;
    return timestamp.toLocaleDateString();
  };

  return (
    <motion.div
      initial={{ opacity: 0, y: 10 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -10 }}
      className={`group relative ${
        isNested ? 'ml-8 mt-2' : 'mb-4'
      } ${comment.resolved ? 'opacity-60' : ''}`}
    >
      <div className="flex gap-3">
        <Avatar className="w-8 h-8 flex-shrink-0">
          <div className="w-full h-full bg-gradient-to-br from-purple-400 to-blue-500 flex items-center justify-center text-white text-sm font-medium">
            {comment.author.charAt(0).toUpperCase()}
          </div>
        </Avatar>
        
        <div className="flex-1 min-w-0">
          <div className="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-3 shadow-sm">
            <div className="flex items-center justify-between mb-2">
              <div className="flex items-center gap-2">
                <span className="font-medium text-sm text-gray-900 dark:text-gray-100">
                  {comment.author}
                </span>
                <span className="text-xs text-gray-500 dark:text-gray-400">
                  {formatTimestamp(comment.timestamp)}
                </span>
                {comment.resolved && (
                  <span className="text-xs bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 px-2 py-1 rounded">
                    Resolved
                  </span>
                )}
              </div>
              
              <div className="relative">
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => setShowActions(!showActions)}
                  className="opacity-0 group-hover:opacity-100 transition-opacity p-1 h-auto"
                >
                  <MoreHorizontal className="w-4 h-4" />
                </Button>
                
                <AnimatePresence>
                  {showActions && (
                    <motion.div
                      initial={{ opacity: 0, scale: 0.95 }}
                      animate={{ opacity: 1, scale: 1 }}
                      exit={{ opacity: 0, scale: 0.95 }}
                      className="absolute right-0 top-8 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg shadow-lg py-1 z-10 min-w-32"
                    >
                      {comment.author === currentUser && (
                        <>
                          <button
                            onClick={() => {
                              setIsEditing(true);
                              setShowActions(false);
                            }}
                            className="w-full text-left px-3 py-2 text-sm hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-2"
                          >
                            <Edit3 className="w-4 h-4" />
                            Edit
                          </button>
                          <button
                            onClick={() => {
                              onDelete();
                              setShowActions(false);
                            }}
                            className="w-full text-left px-3 py-2 text-sm hover:bg-gray-100 dark:hover:bg-gray-700 text-red-600 dark:text-red-400 flex items-center gap-2"
                          >
                            <Trash2 className="w-4 h-4" />
                            Delete
                          </button>
                        </>
                      )}
                      {!comment.resolved && (
                        <button
                          onClick={() => {
                            onResolve();
                            setShowActions(false);
                          }}
                          className="w-full text-left px-3 py-2 text-sm hover:bg-gray-100 dark:hover:bg-gray-700 text-green-600 dark:text-green-400 flex items-center gap-2"
                        >
                          <Check className="w-4 h-4" />
                          Resolve
                        </button>
                      )}
                    </motion.div>
                  )}
                </AnimatePresence>
              </div>
            </div>
            
            {isEditing ? (
              <div className="space-y-2">
                <textarea
                  ref={editInputRef}
                  value={editContent}
                  onChange={(e) => setEditContent(e.target.value)}
                  className="w-full p-2 border border-gray-300 dark:border-gray-600 rounded resize-none bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100"
                  rows={3}
                  placeholder="Edit your comment..."
                />
                <div className="flex gap-2">
                  <Button size="sm" onClick={handleEdit}>
                    Save
                  </Button>
                  <Button size="sm" variant="outline" onClick={() => setIsEditing(false)}>
                    Cancel
                  </Button>
                </div>
              </div>
            ) : (
              <p className="text-sm text-gray-700 dark:text-gray-300 whitespace-pre-wrap">
                {comment.content}
              </p>
            )}
          </div>
          
          {!isNested && !comment.resolved && (
            <div className="mt-2 flex gap-2">
              <Button
                variant="ghost"
                size="sm"
                onClick={() => setIsReplying(true)}
                className="text-xs h-auto py-1 px-2"
              >
                <Reply className="w-3 h-3 mr-1" />
                Reply
              </Button>
            </div>
          )}
          
          <AnimatePresence>
            {isReplying && (
              <motion.div
                initial={{ opacity: 0, height: 0 }}
                animate={{ opacity: 1, height: 'auto' }}
                exit={{ opacity: 0, height: 0 }}
                className="mt-3 space-y-2"
              >
                <textarea
                  ref={replyInputRef}
                  value={replyContent}
                  onChange={(e) => setReplyContent(e.target.value)}
                  className="w-full p-2 border border-gray-300 dark:border-gray-600 rounded resize-none bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100"
                  rows={3}
                  placeholder="Write a reply..."
                />
                <div className="flex gap-2">
                  <Button size="sm" onClick={handleReply}>
                    Reply
                  </Button>
                  <Button size="sm" variant="outline" onClick={() => setIsReplying(false)}>
                    Cancel
                  </Button>
                </div>
              </motion.div>
            )}
          </AnimatePresence>
          
          {comment.replies && comment.replies.length > 0 && (
            <div className="mt-3">
              {comment.replies.map((reply) => (
                <CommentThread
                  key={reply.id}
                  comment={reply}
                  onReply={() => {}} // Nested replies not supported for simplicity
                  onResolve={() => {}}
                  onDelete={() => onDelete()}
                  onEdit={(content) => onEdit(content)}
                  currentUser={currentUser}
                  isNested
                />
              ))}
            </div>
          )}
        </div>
      </div>
    </motion.div>
  );
};

const CommentSystem: React.FC<CommentSystemProps> = ({
  documentId,
  comments,
  onAddComment,
  onReplyToComment,
  onResolveComment,
  onDeleteComment,
  onEditComment,
  currentUser
}) => {
  const [isAddingComment, setIsAddingComment] = useState(false);
  const [newCommentContent, setNewCommentContent] = useState('');
  const [showResolved, setShowResolved] = useState(false);
  const newCommentInputRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    if (isAddingComment && newCommentInputRef.current) {
      newCommentInputRef.current.focus();
    }
  }, [isAddingComment]);

  const handleAddComment = () => {
    if (newCommentContent.trim()) {
      // For now, add comment at line 1, column 1
      // In a real implementation, this would be based on cursor position
      onAddComment(newCommentContent.trim(), { line: 1, column: 1 });
      setNewCommentContent('');
      setIsAddingComment(false);
    }
  };

  const activeComments = comments.filter(comment => !comment.resolved);
  const resolvedComments = comments.filter(comment => comment.resolved);
  const displayComments = showResolved ? comments : activeComments;

  return (
    <div className="h-full flex flex-col bg-gray-50 dark:bg-gray-900">
      {/* Header */}
      <div className="p-4 border-b border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800">
        <div className="flex items-center justify-between mb-3">
          <h3 className="text-lg font-semibold text-gray-900 dark:text-gray-100 flex items-center gap-2">
            <MessageCircle className="w-5 h-5" />
            Comments
          </h3>
          <Button
            onClick={() => setIsAddingComment(true)}
            size="sm"
            className="flex items-center gap-2"
          >
            <Plus className="w-4 h-4" />
            Add Comment
          </Button>
        </div>
        
        <div className="flex items-center gap-4 text-sm text-gray-600 dark:text-gray-400">
          <span>{activeComments.length} active</span>
          <span>{resolvedComments.length} resolved</span>
          <label className="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              checked={showResolved}
              onChange={(e) => setShowResolved(e.target.checked)}
              className="rounded"
            />
            Show resolved
          </label>
        </div>
      </div>
      
      {/* Comment List */}
      <div className="flex-1 overflow-y-auto p-4">
        <AnimatePresence>
          {isAddingComment && (
            <motion.div
              initial={{ opacity: 0, height: 0 }}
              animate={{ opacity: 1, height: 'auto' }}
              exit={{ opacity: 0, height: 0 }}
              className="mb-4 bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-3"
            >
              <div className="space-y-3">
                <div className="flex items-center gap-2">
                  <Avatar className="w-8 h-8">
                    <div className="w-full h-full bg-gradient-to-br from-purple-400 to-blue-500 flex items-center justify-center text-white text-sm font-medium">
                      {currentUser.charAt(0).toUpperCase()}
                    </div>
                  </Avatar>
                  <span className="font-medium text-sm text-gray-900 dark:text-gray-100">
                    {currentUser}
                  </span>
                </div>
                <textarea
                  ref={newCommentInputRef}
                  value={newCommentContent}
                  onChange={(e) => setNewCommentContent(e.target.value)}
                  className="w-full p-2 border border-gray-300 dark:border-gray-600 rounded resize-none bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100"
                  rows={3}
                  placeholder="Write a comment..."
                />
                <div className="flex gap-2">
                  <Button size="sm" onClick={handleAddComment}>
                    Add Comment
                  </Button>
                  <Button size="sm" variant="outline" onClick={() => setIsAddingComment(false)}>
                    Cancel
                  </Button>
                </div>
              </div>
            </motion.div>
          )}
        </AnimatePresence>
        
        {displayComments.length === 0 ? (
          <div className="text-center py-8 text-gray-500 dark:text-gray-400">
            <MessageCircle className="w-12 h-12 mx-auto mb-3 opacity-50" />
            <p>No comments yet</p>
            <p className="text-sm">Add a comment to start the conversation</p>
          </div>
        ) : (
          <div className="space-y-4">
            {displayComments.map((comment) => (
              <CommentThread
                key={comment.id}
                comment={comment}
                onReply={(content) => onReplyToComment(comment.id, content)}
                onResolve={() => onResolveComment(comment.id)}
                onDelete={() => onDeleteComment(comment.id)}
                onEdit={(content) => onEditComment(comment.id, content)}
                currentUser={currentUser}
              />
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export default CommentSystem;
export type { Comment };