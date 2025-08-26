import { useTranslation } from "react-i18next";
import React from "react";
import { Dropdown } from "../ui/Dropdown";
import { Home, ChevronDown, HelpCircle, Cog, Sparkles } from "lucide-react";
import { useUIStore } from "../../stores/uiStore";

type Item = { label: string; value: string; selected?: boolean };

const PillButton: React.FC<{ label: string; className?: string }> = ({
  label,
  className = "",
}) => (
  <button
    className={
      "inline-flex items-center gap-1 rounded-full px-3 py-1.5 text-sm font-medium shadow-sm border " +
      "border-slate-200/70 bg-white/90 hover:bg-white dark:bg-slate-800/80 dark:border-slate-700 " +
      "transition-colors " +
      className
    }
  >
    <span>{label}</span>
    <ChevronDown size={16} className="opacity-70" />
  </button>
);

const Pill: React.FC<{
  label: string;
  items: Item[];
  onSelect?: (v: string) => void;
  className?: string;
}> = ({ label, items, onSelect, className }) => {
  return (
    <Dropdown
      trigger={<PillButton label={label} className={className} />}
      items={items}
      onSelect={(v) => onSelect?.(v)}
      className="z-40"
    />
  );
};

const TopBar: React.FC = () => {
  const { wordCount, saveStatus, openAIPanel, setAIActiveTool } = useUIStore();
  const { t } = useTranslation("ui");

  const openTool = (tool: any) => {
    setAIActiveTool(tool);
    openAIPanel(tool);
  };
  const writeItems: Item[] = [
    { label: "Auto", value: "auto", selected: true },
    { label: "Guided", value: "guided" },
    { label: "Tone Shift", value: "tone" },
    { label: "Write settings", value: "settings" },
  ];

  const rewriteItems: Item[] = [
    { label: "Rewrite sentence", value: "sentence" },
    { label: "Rewrite paragraph", value: "paragraph" },
    { label: "Shorten", value: "shorten" },
    { label: "Expand", value: "expand" },
  ];

  const describeItems: Item[] = [
    { label: "Sight", value: "sight" },
    { label: "Smell", value: "smell" },
    { label: "Taste", value: "taste" },
    { label: "Sound", value: "sound" },
    { label: "Touch", value: "touch" },
    { label: "Metaphor", value: "metaphor" },
  ];

  const brainstormItems: Item[] = [
    { label: "First Draft", value: "first_draft" },
    { label: "Shrink Ray", value: "shrink_ray" },
    { label: "Twist", value: "twist" },
    { label: "Characters", value: "characters" },
    { label: "Poem", value: "poem" },
    { label: "Visualize", value: "visualize" },
    { label: "Feedback", value: "feedback" },
  ];

  return (
    <div
      className={
        "sticky top-0 z-30 w-full border-b border-slate-200/70 dark:border-slate-800 " +
        "bg-gradient-to-r from-pink-100/80 via-white/80 to-purple-100/80 " +
        "backdrop-blur supports-[backdrop-filter]:bg-white/60"
      }
    >
      <div className="mx-auto max-w-screen-2xl px-4 py-2">
        <div className="flex items-center justify-between gap-3">
          {/* Left cluster */}
          <div className="flex items-center gap-2">
            <button
              aria-label="Home"
              className="rounded-full p-2 bg-white/90 border border-slate-200/70 shadow-sm hover:bg-white dark:bg-slate-800/80 dark:border-slate-700"
              title="Home"
            >
              <Home size={18} />
            </button>

            <div className="mx-1 h-6 w-px bg-slate-300/60 dark:bg-slate-700" />

            <Pill label="Write" items={writeItems} onSelect={() => openTool('write')} />
            <Pill label="Rewrite" items={rewriteItems} onSelect={() => openTool('rewrite')} />
            <Pill label="Describe" items={describeItems} onSelect={() => openTool('describe')} />
            <Pill
              label="Brainstorm"
              items={brainstormItems}
              onSelect={() => openTool('brainstorm')}
              className="pl-2 pr-2"
            />

            {/* Plugins pill (decorative) */}
            <div className="hidden md:flex">
              <Dropdown
                trigger={
                  <button className="inline-flex items-center gap-1 rounded-full px-3 py-1.5 text-sm font-medium shadow-sm border border-slate-200/70 bg-white/90 hover:bg-white dark:bg-slate-800/80 dark:border-slate-700">
                    <Sparkles size={16} className="text-pink-500" />
                    <span>Plugins</span>
                    <ChevronDown size={16} className="opacity-70" />
                  </button>
                }
                items={[
                  { label: "Explore Plugins", value: "explore" },
                  { label: "Reorder this Menu...", value: "reorder" },
                ]}
                onSelect={() => {}}
              />
            </div>
          </div>

          {/* Right cluster */}
          <div className="flex items-center gap-2 text-sm">
            <span className="px-2 py-1 rounded-full bg-white/70 border border-slate-200/70 shadow-sm dark:bg-slate-800/80 dark:border-slate-700">
              {`Words: ${wordCount}`}
            </span>
            <span className="px-2 py-1 rounded-full bg-white/70 border border-slate-200/70 shadow-sm dark:bg-slate-800/80 dark:border-slate-700">
              {saveStatus === 'saving' ? 'Saving…' : saveStatus === 'error' ? 'Error' : 'Saved ✓'}
            </span>
            <button className="px-2 py-1 rounded-full bg-white/70 border border-slate-200/70 shadow-sm hover:bg-white dark:bg-slate-800/80 dark:border-slate-700">
              {t("whats_new")}
            </button>
            <button
              aria-label="Help"
              className="rounded-full p-2 bg-white/90 border border-slate-200/70 shadow-sm hover:bg-white dark:bg-slate-800/80 dark:border-slate-700"
              title="Help"
            >
              <HelpCircle size={18} />
            </button>
            <button
              aria-label="Settings"
              className="rounded-full p-2 bg-white/90 border border-slate-200/70 shadow-sm hover:bg-white dark:bg-slate-800/80 dark:border-slate-700"
              title="Settings"
            >
              <Cog size={18} />
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default TopBar;
