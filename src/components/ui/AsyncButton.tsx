import React, { useState } from 'react';
import { Loader2 } from 'lucide-react';
import { cn } from '../../utils/cn';
import { useErrorHandler } from '../../hooks/useErrorHandler';

export interface AsyncButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  onClick?: (event: React.MouseEvent<HTMLButtonElement>) => Promise<void> | void;
  loadingText?: string;
  variant?: 'primary' | 'secondary' | 'danger' | 'ghost';
  size?: 'sm' | 'md' | 'lg';
  showErrorToast?: boolean;
  onError?: (error: Error) => void;
}

const variantClasses = {
  primary: 'bg-blue-600 hover:bg-blue-700 text-white border-transparent',
  secondary: 'bg-gray-200 hover:bg-gray-300 text-gray-900 border-gray-300',
  danger: 'bg-red-600 hover:bg-red-700 text-white border-transparent',
  ghost: 'bg-transparent hover:bg-gray-100 text-gray-700 border-gray-300',
};

const sizeClasses = {
  sm: 'px-3 py-1.5 text-sm',
  md: 'px-4 py-2 text-base',
  lg: 'px-6 py-3 text-lg',
};

export const AsyncButton: React.FC<AsyncButtonProps> = ({
  children,
  onClick,
  loadingText = 'Loading...',
  variant = 'primary',
  size = 'md',
  showErrorToast = true,
  onError,
  disabled,
  className,
  ...props
}) => {
  const [isLoading, setIsLoading] = useState(false);
  const { handleError } = useErrorHandler();

  const handleClick = async (event: React.MouseEvent<HTMLButtonElement>) => {
    if (!onClick || isLoading) return;

    try {
      setIsLoading(true);
      await onClick(event);
    } catch (error) {
      const errorObj = error instanceof Error ? error : new Error(String(error));
      
      if (showErrorToast) {
        handleError(errorObj, { component: 'AsyncButton', action: 'onClick' });
      }
      
      onError?.(errorObj);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <button
      {...props}
      onClick={handleClick}
      disabled={disabled || isLoading}
      className={cn(
        'inline-flex items-center justify-center gap-2 rounded-md border font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed',
        variantClasses[variant],
        sizeClasses[size],
        className
      )}
    >
      {isLoading && <Loader2 className="w-4 h-4 animate-spin" />}
      {isLoading ? loadingText : children}
    </button>
  );
};

export default AsyncButton;