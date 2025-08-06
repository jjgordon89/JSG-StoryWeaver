import React, { useState, useRef, useEffect } from 'react';
import { Button } from './Button';

interface DropdownProps {
  trigger: React.ReactNode;
  items: {
    label: string;
    value: string;
    selected?: boolean;
  }[];
  onSelect: (value: string) => void;
  className?: string;
}

export const Dropdown: React.FC<DropdownProps> = ({
  trigger,
  items,
  onSelect,
  className = '',
}) => {
  const [isOpen, setIsOpen] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);

  const handleSelect = (value: string) => {
    onSelect(value);
    setIsOpen(false);
  };

  // Close dropdown when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, []);

  return (
    <div className={`relative inline-block ${className}`} ref={dropdownRef}>
      <div onClick={() => setIsOpen(!isOpen)}>
        {trigger}
      </div>
      
      {isOpen && (
        <div className="absolute right-0 mt-1 w-48 bg-white dark:bg-slate-800 rounded-md shadow-lg border border-slate-200 dark:border-slate-700 z-10">
          {items.map((item) => (
            <button
              key={item.value}
              className={`block w-full text-left px-4 py-2 text-sm hover:bg-slate-100 dark:hover:bg-slate-700 ${
                item.selected ? 'bg-slate-100 dark:bg-slate-700' : ''
              }`}
              onClick={() => handleSelect(item.value)}
            >
              {item.label}
            </button>
          ))}
        </div>
      )}
    </div>
  );
};

export const DropdownButton: React.FC<{
  label: string;
  items: { label: string; value: string; selected?: boolean }[];
  onSelect: (value: string) => void;
  icon?: React.ReactNode;
  variant?: 'default' | 'primary' | 'secondary' | 'ghost' | 'link' | 'outline';
  size?: 'default' | 'sm' | 'lg' | 'icon';
  className?: string;
}> = ({ label, items, onSelect, icon, variant = 'outline', size = 'sm', className = '' }) => {
  return (
    <Dropdown
      trigger={
        <Button variant={variant} size={size} className={`flex items-center ${className}`}>
          {icon && <span className="mr-1">{icon}</span>}
          {label}
        </Button>
      }
      items={items}
      onSelect={onSelect}
    />
  );
};
