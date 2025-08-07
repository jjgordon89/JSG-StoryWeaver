import React from 'react';

interface CardProps {
  className?: string;
  children: React.ReactNode;
  onClick?: () => void;
  onMouseEnter?: () => void;
  onMouseLeave?: () => void;
}

export const Card: React.FC<CardProps> = ({ 
  className = '', 
  children, 
  onClick,
  onMouseEnter,
  onMouseLeave 
}) => {
  return (
    <div 
      className={`rounded-lg border border-slate-200 dark:border-slate-700 bg-white dark:bg-slate-900 shadow-sm ${className}`}
      onClick={onClick}
      onMouseEnter={onMouseEnter}
      onMouseLeave={onMouseLeave}
    >
      {children}
    </div>
  );
};

export const CardHeader: React.FC<CardProps> = ({ className = '', children }) => {
  return (
    <div className={`px-4 py-3 border-b border-slate-200 dark:border-slate-700 ${className}`}>
      {children}
    </div>
  );
};

export const CardContent: React.FC<CardProps> = ({ className = '', children }) => {
  return (
    <div className={`px-4 py-3 ${className}`}>
      {children}
    </div>
  );
};

export const CardTitle: React.FC<CardProps> = ({ className = '', children }) => {
  return (
    <h3 className={`text-lg font-semibold leading-none tracking-tight ${className}`}>
      {children}
    </h3>
  );
};

export const CardFooter: React.FC<CardProps> = ({ className = '', children }) => {
  return (
    <div className={`px-4 py-3 border-t border-slate-200 dark:border-slate-700 ${className}`}>
      {children}
    </div>
  );
};
