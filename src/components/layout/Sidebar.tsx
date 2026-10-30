import { useTranslation } from "react-i18next";
import { toolGroups, tools } from "@/app/tools";
import { SidebarBrand } from "./SidebarBrand";
import { SidebarFooter } from "./SidebarFooter";
import { SidebarItem } from "./SidebarItem";

interface SidebarProps {
  activeId: string;
  onSelect: (id: string) => void;
}

export function Sidebar({ activeId, onSelect }: SidebarProps) {
  const { t } = useTranslation();

  return (
    <aside className="flex w-[220px] shrink-0 flex-col border-r border-sidebar-border bg-sidebar">
      <SidebarBrand />
      <nav className="flex flex-col gap-4 overflow-y-auto px-3 pb-4">
        {toolGroups.map((group) => (
          <div key={group} className="flex flex-col gap-0.5">
            <h2 className="px-2.5 pb-1 text-[11px] font-semibold text-muted-foreground">
              {t(`groups.${group}`)}
            </h2>
            {tools
              .filter((tool) => tool.group === group)
              .map((tool) => (
                <SidebarItem
                  key={tool.id}
                  icon={tool.icon}
                  label={t(`${tool.i18nKey}.title`)}
                  active={tool.id === activeId}
                  onSelect={() => onSelect(tool.id)}
                />
              ))}
          </div>
        ))}
      </nav>
      <SidebarFooter />
    </aside>
  );
}
