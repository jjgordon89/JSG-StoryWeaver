import React from 'react';

interface ScreenReaderAnnouncerProps {
  message: string;
}

const ScreenReaderAnnouncer: React.FC<ScreenReaderAnnouncerProps> = ({ message }) => {
  return (
    <div
      aria-live="polite"
      aria-atomic="true"
      className="sr-only"
    >
      {message}
    </div>
  );
};

export default ScreenReaderAnnouncer;