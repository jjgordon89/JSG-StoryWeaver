import React, { useEffect, useRef } from 'react';
import SeriesConsistencyWidget from './SeriesConsistencyWidget.svelte';
import SeriesConsistencyReport from './SeriesConsistencyReport.svelte';

interface SeriesConsistencyWidgetProps {
  seriesId: string;
  size?: 'small' | 'medium' | 'large';
  onViewReport?: () => void;
}

interface SeriesConsistencyReportProps {
  seriesId: string;
}

// React wrapper for Svelte SeriesConsistencyWidget
export const SeriesConsistencyWidgetReact: React.FC<SeriesConsistencyWidgetProps> = ({
  seriesId,
  size = 'medium',
  onViewReport
}) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const componentRef = useRef<any>(null);

  useEffect(() => {
    if (containerRef.current && !componentRef.current) {
      componentRef.current = new SeriesConsistencyWidget({
        target: containerRef.current,
        props: {
          seriesId,
          size,
          onViewReport
        }
      });
    }

    return () => {
      if (componentRef.current) {
        componentRef.current.$destroy();
        componentRef.current = null;
      }
    };
  }, []);

  useEffect(() => {
    if (componentRef.current) {
      componentRef.current.$set({
        seriesId,
        size,
        onViewReport
      });
    }
  }, [seriesId, size, onViewReport]);

  return <div ref={containerRef} />;
};

// React wrapper for Svelte SeriesConsistencyReport
export const SeriesConsistencyReportReact: React.FC<SeriesConsistencyReportProps> = ({
  seriesId
}) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const componentRef = useRef<any>(null);

  useEffect(() => {
    if (containerRef.current && !componentRef.current) {
      componentRef.current = new SeriesConsistencyReport({
        target: containerRef.current,
        props: {
          seriesId
        }
      });
    }

    return () => {
      if (componentRef.current) {
        componentRef.current.$destroy();
        componentRef.current = null;
      }
    };
  }, []);

  useEffect(() => {
    if (componentRef.current) {
      componentRef.current.$set({
        seriesId
      });
    }
  }, [seriesId]);

  return <div ref={containerRef} />;
};