import React from 'react';
import { Check } from 'lucide-react';

interface CheckboxProps {
  id?: string;
  checked?: boolean;
  onCheckedChange?: (checked: boolean) => void;
  disabled?: boolean;
  className?: string;
  'aria-label'?: string;
}

const Checkbox: React.FC<CheckboxProps> = ({
  id,
  checked = false,
  onCheckedChange,
  disabled = false,
  className = '',
  'aria-label': ariaLabel,
}) => {
  const handleClick = () => {
    if (!disabled && onCheckedChange) {
      onCheckedChange(!checked);
    }
  };

  return (
    <button
      id={id}
      type="button"
      role="checkbox"
      aria-checked={checked}
      aria-label={ariaLabel}
      disabled={disabled}
      onClick={handleClick}
      className={`
        inline-flex items-center justify-center
        w-4 h-4 border border-gray-300 dark:border-gray-600
        rounded bg-white dark:bg-gray-800
        transition-colors duration-200
        ${checked ? 'bg-blue-600 border-blue-600 text-white' : ''}
        ${disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer hover:border-blue-500'}
        ${className}
      `}
    >
      {checked && (
        <Check className="w-3 h-3" strokeWidth={3} />
      )}
    </button>
  );
};

export { Checkbox };