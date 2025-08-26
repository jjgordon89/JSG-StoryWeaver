import React, { useState, useRef, useEffect } from 'react';
import * as Checkbox from '@radix-ui/react-checkbox';
import { Button } from '../../../../ui/components/common';
import { Card, CardContent, CardHeader, CardTitle } from '../../../../ui/components/common';
import { Input } from '../../../../ui/components/common';
import { Textarea } from '../../../../ui/components/common';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../../../../ui/components/common';
import { Upload, Download, AlertCircle, CheckCircle, X, Check } from 'lucide-react';

interface CSVImportDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onImport: (data: any[], type: 'characters' | 'worldbuilding') => Promise<void>;
  importType: 'characters' | 'worldbuilding';
  projectId: string;
}

interface ParsedRow {
  [key: string]: string;
}

interface ValidationError {
  row: number;
  field: string;
  message: string;
}

const CSVImportDialog: React.FC<CSVImportDialogProps> = ({
  isOpen,
  onClose,
  onImport,
  importType,
  projectId
}) => {
  const [csvContent, setCsvContent] = useState('');
  const [parsedData, setParsedData] = useState<ParsedRow[]>([]);
  const [validationErrors, setValidationErrors] = useState<ValidationError[]>([]);
  const [isProcessing, setIsProcessing] = useState(false);
  const [step, setStep] = useState<'upload' | 'preview' | 'complete'>('upload');
  const [isDragging, setIsDragging] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [selectedRows, setSelectedRows] = useState<Set<number>>(new Set());


  useEffect(() => {
    if (step === 'preview') {
      const allRowIndices = new Set(parsedData.map((_, index) => index));
      setSelectedRows(allRowIndices);
    }
  }, [parsedData, step]);

  // Character CSV template
  const characterTemplate = [
    'Name,Description,Appearance,Personality,Background,Skills,Goals,Flaws,Secrets,Relationships,Visibility,Series Shared',
    'John Doe,"A brave knight","Tall with brown hair","Courageous and loyal","Born in a small village","Swordsmanship, Leadership","Protect the kingdom","Sometimes too trusting","Has a secret fear of heights","Friend of the king","Public","Yes"',
    'Jane Smith,"A wise mage","Short with silver hair","Intelligent and mysterious","Studied at the magic academy","Spellcasting, Alchemy","Master all schools of magic","Arrogant about her abilities","Lost her memory of childhood","Mentor to young wizards","Public","No"'
  ].join('\n');

  // Worldbuilding CSV template
  const worldbuildingTemplate = [
    'Name,Type,Description,Significance,Visibility,Series Shared',
    'The Great Library,"Location","A massive library containing all knowledge","Central to the magic system","Always","Yes"',
    'Order of the Phoenix,"Organization","Secret society of mages","Key to the plot","Chapter","No"',
    'Crystal of Power,"Artifact","Ancient crystal with immense magical energy","Main plot device","Never","Yes"'
  ].join('\n');

  const downloadTemplate = () => {
    const template = importType === 'characters' ? characterTemplate : worldbuildingTemplate;
    const filename = importType === 'characters' ? 'character_import_template.csv' : 'worldbuilding_import_template.csv';
    
    const blob = new Blob([template], { type: 'text/csv;charset=utf-8;' });
    const link = document.createElement('a');
    const url = URL.createObjectURL(blob);
    link.setAttribute('href', url);
    link.setAttribute('download', filename);
    link.style.visibility = 'hidden';
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  };

  const handleFile = (file: File) => {
    if (!file) return;

    if (!file.name.toLowerCase().endsWith('.csv')) {
      alert('Please select a CSV file');
      return;
    }

    const reader = new FileReader();
    reader.onload = (e) => {
      const content = e.target?.result as string;
      setCsvContent(content);
      parseCSV(content);
    };
    reader.readAsText(file);
  };

  const handleDragOver = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    setIsDragging(true);
  };

  const handleDragLeave = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    setIsDragging(false);
  };

  const handleDrop = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    setIsDragging(false);
    if (e.dataTransfer.files && e.dataTransfer.files.length > 0) {
      handleFile(e.dataTransfer.files[0]);
      e.dataTransfer.clearData();
    }
  };

  const handleFileUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (file) {
      handleFile(file);
    }
  };

  const parseCSV = (content: string) => {
    try {
      const lines = content.trim().split('\n');
      if (lines.length < 2) {
        throw new Error('CSV must have at least a header row and one data row');
      }

      // Parse header
      const headers = lines[0].split(',').map(h => h.trim().replace(/"/g, ''));
      
      // Parse data rows
      const data: ParsedRow[] = [];
      const errors: ValidationError[] = [];

      for (let i = 1; i < lines.length; i++) {
        const line = lines[i].trim();
        if (!line) continue;

        // Simple CSV parsing (handles quoted fields)
        const values = parseCSVLine(line);
        
        if (values.length !== headers.length) {
          errors.push({
            row: i + 1,
            field: 'general',
            message: `Row has ${values.length} columns but expected ${headers.length}`
          });
          continue;
        }

        const row: ParsedRow = {};
        headers.forEach((header, index) => {
          row[header] = values[index] || '';
        });

        // Validate required fields
        if (importType === 'characters') {
          if (!row['Name'] || row['Name'].trim() === '') {
            errors.push({
              row: i + 1,
              field: 'Name',
              message: 'Character name is required'
            });
          }
        } else if (importType === 'worldbuilding') {
          if (!row['Name'] || row['Name'].trim() === '') {
            errors.push({
              row: i + 1,
              field: 'Name',
              message: 'Element name is required'
            });
          }
          if (!row['Type'] || row['Type'].trim() === '') {
            errors.push({
              row: i + 1,
              field: 'Type',
              message: 'Element type is required'
            });
          }
        }

        data.push(row);
      }

      setParsedData(data);
      setValidationErrors(errors);
      setStep('preview');
    } catch (error) {
      alert(`Error parsing CSV: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  };

  const parseCSVLine = (line: string): string[] => {
    const result: string[] = [];
    let current = '';
    let inQuotes = false;
    
    for (let i = 0; i < line.length; i++) {
      const char = line[i];
      
      if (char === '"') {
        if (inQuotes && line[i + 1] === '"') {
          // Escaped quote
          current += '"';
          i++; // Skip next quote
        } else {
          // Toggle quote state
          inQuotes = !inQuotes;
        }
      } else if (char === ',' && !inQuotes) {
        // End of field
        result.push(current.trim());
        current = '';
      } else {
        current += char;
      }
    }
    
    // Add the last field
    result.push(current.trim());
    return result;
  };

  const handleSelectAll = (checked: boolean) => {
    if (checked) {
      const allRowIndices = new Set(parsedData.map((_, index) => index));
      setSelectedRows(allRowIndices);
    } else {
      setSelectedRows(new Set());
    }
  };

  const handleSelectRow = (index: number, checked: boolean) => {
    const newSelectedRows = new Set(selectedRows);
    if (checked) {
      newSelectedRows.add(index);
    } else {
      newSelectedRows.delete(index);
    }
    setSelectedRows(newSelectedRows);
  };

  const handleImport = async () => {
    if (validationErrors.length > 0) {
      const proceed = confirm(
        `There are ${validationErrors.length} validation errors. Do you want to proceed with importing valid rows only?`
      );
      if (!proceed) return;
    }

    setIsProcessing(true);
    try {
      // Filter out rows with errors and unselected rows
      const validRows = parsedData.filter((_, index) => {
        const rowNumber = index + 2; // +2 because index is 0-based and we skip header
        const isSelected = selectedRows.has(index);
        const hasError = validationErrors.some(error => error.row === rowNumber);
        return isSelected && !hasError;
      });

      // Transform data based on import type
      let transformedData;
      if (importType === 'characters') {
        transformedData = validRows.map(row => ({
          name: row['Name'],
          description: row['Description'] || '',
          traits: {
            appearance: row['Appearance'] || '',
            personality: row['Personality'] || '',
            background: row['Background'] || '',
            skills: row['Skills'] || '',
            goals: row['Goals'] || '',
            flaws: row['Flaws'] || '',
            secrets: row['Secrets'] || '',
            relationships: row['Relationships'] || ''
          },
          visibility: row['Visibility']?.toLowerCase() === 'private' ? 'private' : 'public',
          series_shared: row['Series Shared']?.toLowerCase() === 'yes'
        }));
      } else {
        transformedData = validRows.map(row => ({
          name: row['Name'],
          element_type: row['Type']?.toLowerCase() || 'other',
          description: row['Description'] || '',
          significance: row['Significance'] || '',
          visibility: getVisibilityValue(row['Visibility']),
          series_shared: row['Series Shared']?.toLowerCase() === 'yes'
        }));
      }

      await onImport(transformedData, importType);
      setStep('complete');
    } catch (error) {
      alert(`Import failed: ${error instanceof Error ? error.message : 'Unknown error'}`);
    } finally {
      setIsProcessing(false);
    }
  };

  const getVisibilityValue = (visibility: string): 'always' | 'chapter' | 'never' => {
    const lower = visibility?.toLowerCase();
    if (lower === 'chapter') return 'chapter';
    if (lower === 'never') return 'never';
    return 'always';
  };

  const resetDialog = () => {
    setCsvContent('');
    setParsedData([]);
    setValidationErrors([]);
    setStep('upload');
    setSelectedRows(new Set());
    if (fileInputRef.current) {
      fileInputRef.current.value = '';
    }
  };

  const handleClose = () => {
    resetDialog();
    onClose();
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg shadow-xl max-w-4xl w-full mx-4 max-h-[90vh] overflow-hidden">
        <div className="flex items-center justify-between p-6 border-b">
          <h2 className="text-xl font-semibold">
            Import {importType === 'characters' ? 'Characters' : 'Worldbuilding Elements'} from CSV
          </h2>
          <Button variant="ghost" onClick={handleClose}>
            <X className="h-4 w-4" />
          </Button>
        </div>

        <div className="p-6 overflow-y-auto max-h-[calc(90vh-120px)]">
          {step === 'upload' && (
            <div className="space-y-6">
              <div
                className={`border-2 border-dashed rounded-lg p-8 text-center transition-colors ${isDragging ? 'border-blue-500 bg-blue-50' : 'border-gray-300'}`}
                onDragOver={handleDragOver}
                onDragLeave={handleDragLeave}
                onDrop={handleDrop}
              >
                <Upload className="h-12 w-12 text-gray-400 mx-auto mb-4" />
                <h3 className="text-lg font-medium text-gray-900 mb-2">
                  Drag and drop your CSV file here
                </h3>
                <p className="text-sm text-gray-500">
                  to import {importType === 'characters' ? 'characters' : 'worldbuilding elements'}
                </p>
              </div>

              <div className="relative my-4">
                <div className="absolute inset-0 flex items-center" aria-hidden="true">
                  <div className="w-full border-t border-gray-300" />
                </div>
                <div className="relative flex justify-center">
                  <span className="bg-white px-2 text-sm text-gray-500">OR</span>
                </div>
              </div>

              <div className="text-center">
                <input
                  ref={fileInputRef}
                  type="file"
                  accept=".csv"
                  onChange={handleFileUpload}
                  className="hidden"
                />
                <Button variant="outline" onClick={() => fileInputRef.current?.click()}>
                  <Upload className="h-4 w-4 mr-2" />
                  Select File From Your Computer
                </Button>
              </div>

              <div className="text-center">
                <p className="text-gray-600 mb-2">Or paste CSV content directly:</p>
                <Textarea
                  value={csvContent}
                  onChange={(e) => setCsvContent(e.target.value)}
                  placeholder="Paste your CSV content here..."
                  rows={8}
                  className="font-mono text-sm"
                />
                {csvContent && (
                  <Button 
                    onClick={() => parseCSV(csvContent)}
                    className="mt-2"
                  >
                    Parse CSV
                  </Button>
                )}
              </div>

              <div className="bg-blue-50 p-4 rounded-lg">
                <h4 className="font-medium text-blue-900 mb-2">
                  Need a template?
                </h4>
                <p className="text-blue-800 text-sm mb-3">
                  Download our CSV template to get started with the correct format.
                </p>
                <Button variant="outline" onClick={downloadTemplate}>
                  <Download className="h-4 w-4 mr-2" />
                  Download Template
                </Button>
              </div>
            </div>
          )}

          {step === 'preview' && (
            <div className="space-y-6">
              <div className="flex items-center justify-between">
                <h3 className="text-lg font-medium">
                  Preview Import Data
                </h3>
                <div className="flex items-center gap-2">
                  <span className="text-sm text-gray-600">
                    {selectedRows.size} of {parsedData.length} rows selected
                  </span>
                  {validationErrors.length > 0 && (
                    <span className="text-sm text-red-600">
                      {validationErrors.length} errors
                    </span>
                  )}
                </div>
              </div>

              {validationErrors.length > 0 && (
                <Card className="border-red-200">
                  <CardHeader>
                    <CardTitle className="text-red-800 flex items-center gap-2">
                      <AlertCircle className="h-4 w-4" />
                      Validation Errors
                    </CardTitle>
                  </CardHeader>
                  <CardContent>
                    <div className="space-y-2 max-h-32 overflow-y-auto">
                      {validationErrors.map((error, index) => (
                        <div key={index} className="text-sm text-red-700">
                          Row {error.row}, {error.field}: {error.message}
                        </div>
                      ))}
                    </div>
                  </CardContent>
                </Card>
              )}

              <div className="border rounded-lg overflow-hidden">
                <div className="overflow-x-auto max-h-96">
                  <table className="w-full text-sm">
                    <thead className="bg-gray-50">
                      <tr>
                        <th className="px-3 py-2 text-left font-medium text-gray-900">
                          <label htmlFor="selectAll" className="flex items-center space-x-2">
                            <Checkbox.Root
                              id="selectAll"
                              checked={selectedRows.size === parsedData.length && parsedData.length > 0}
                              onCheckedChange={(checked: boolean) => handleSelectAll(checked)}
                              className="flex h-4 w-4 items-center justify-center rounded border border-gray-300 focus:ring-2 focus:ring-blue-500"
                            >
                              <Checkbox.Indicator>
                                <Check className="h-3 w-3 text-blue-600" />
                              </Checkbox.Indicator>
                            </Checkbox.Root>
                            <span className="sr-only">Select all rows</span>
                          </label>
                        </th>
                        {parsedData.length > 0 && Object.keys(parsedData[0]).map(header => (
                          <th key={header} className="px-3 py-2 text-left font-medium text-gray-900">
                            {header}
                          </th>
                        ))}
                      </tr>
                    </thead>
                    <tbody>
                      {parsedData.slice(0, 10).map((row, index) => {
                        const rowNumber = index + 2;
                        const hasError = validationErrors.some(error => error.row === rowNumber);
                        const isSelected = selectedRows.has(index);

                        return (
                          <tr key={index} className={`${hasError ? 'bg-red-50' : ''} ${isSelected ? '' : 'bg-gray-100 opacity-60'}`}>
                             <td className="px-3 py-2 border-t">
                              <label htmlFor={`select-row-${index}`} className="flex items-center space-x-2">
                                <Checkbox.Root
                                 id={`select-row-${index}`}
                                 checked={isSelected}
                                 onCheckedChange={(checked: boolean) => handleSelectRow(index, checked)}
                                 disabled={hasError}
                                 className="flex h-4 w-4 items-center justify-center rounded border border-gray-300 focus:ring-2 focus:ring-blue-500 disabled:opacity-50"
                               >
                                 <Checkbox.Indicator>
                                   <Check className="h-3 w-3 text-blue-600" />
                                 </Checkbox.Indicator>
                               </Checkbox.Root>
                               <span className="sr-only">Select row {index + 1}</span>
                              </label>
                            </td>
                            {Object.values(row).map((value, cellIndex) => (
                              <td key={cellIndex} className="px-3 py-2 border-t">
                                {value}
                              </td>
                            ))}
                          </tr>
                        );
                      })}
                    </tbody>
                  </table>
                </div>
                {parsedData.length > 10 && (
                  <div className="p-3 bg-gray-50 text-sm text-gray-600 text-center">
                    Showing first 10 rows of {parsedData.length} total rows
                  </div>
                )}
              </div>

              <div className="flex justify-between">
                <Button variant="outline" onClick={() => setStep('upload')}>
                  Back
                </Button>
                <Button 
                  onClick={handleImport}
                  disabled={isProcessing || parsedData.length === 0}
                >
                  {isProcessing ? 'Importing...' : `Import ${selectedRows.size} Items`}
                </Button>
              </div>
            </div>
          )}

          {step === 'complete' && (
            <div className="text-center space-y-6">
              <div className="flex justify-center">
                <CheckCircle className="h-16 w-16 text-green-500" />
              </div>
              <div>
                <h3 className="text-lg font-medium text-gray-900 mb-2">
                  Import Complete!
                </h3>
                <p className="text-gray-600">
                  Successfully imported {parsedData.length - validationErrors.length} {importType === 'characters' ? 'characters' : 'worldbuilding elements'}.
                </p>
              </div>
              <Button onClick={handleClose}>
                Close
              </Button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default CSVImportDialog;