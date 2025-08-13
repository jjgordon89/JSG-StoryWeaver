import React from 'react';
import SeriesConsistencyWidget from './SeriesConsistencyWidget';
import SeriesConsistencyReportComponent from './SeriesConsistencyReport';

interface SeriesConsistencyWidgetProps {
  seriesId: string;
  size?: 'sm' | 'md' | 'lg';
  showDetails?: boolean;
  onViewReport?: () => void;
}

interface SeriesConsistencyReportProps {
  seriesId: string;
  seriesName?: string;
}

// Direct React component exports (no longer wrappers)
export const SeriesConsistencyWidgetReact: React.FC<SeriesConsistencyWidgetProps> = ({
  seriesId,
  size = 'md',
  showDetails = true,
  onViewReport
}) => {
  return (
    <SeriesConsistencyWidget
      seriesId={seriesId}
      size={size}
      showDetails={showDetails}
      onViewReport={onViewReport}
    />
  );
};

export const SeriesConsistencyReportReact: React.FC<SeriesConsistencyReportProps> = ({
  seriesId,
  seriesName
}) => {
  return (
    <SeriesConsistencyReportComponent
      seriesId={seriesId}
      seriesName={seriesName}
    />
  );
};
