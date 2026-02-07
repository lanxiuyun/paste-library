"use client"

import { useState, useEffect, useRef } from "react"
import { Search, Bell, Sparkles, Copy, Star, Trash2 } from "lucide-react"
import { cn } from "@/lib/utils"

type ClipboardItem = {
  id: number
  type: "HTML" | "纯文本" | "图片"
  content: string
  charCount: number
  time: string
  starred?: boolean
}

const mockData: ClipboardItem[] = [
  {
    id: 1,
    type: "HTML",
    content: "估计第一名10W 第二名5W 第三名2W 这种",
    charCount: 110,
    time: "1小时前",
  },
  {
    id: 2,
    type: "纯文本",
    content: "你可以自定义如何来渲染每一个选项。",
    charCount: 17,
    time: "1小时前",
  },
  {
    id: 3,
    type: "HTML",
    content: "你可以自定义如何来渲染每一个选项。",
    charCount: 773,
    time: "1小时前",
  },
  {
    id: 4,
    type: "纯文本",
    content: "React 组件开发最佳实践指南",
    charCount: 256,
    time: "2小时前",
  },
  {
    id: 5,
    type: "HTML",
    content: "const App = () => { return <div>Hello</div> }",
    charCount: 45,
    time: "3小时前",
    starred: true,
  },
]

const tabs = ["全部", "文本", "图片", "文件", "收藏"]

export default function ClipboardManager() {
  const [activeTab, setActiveTab] = useState("全部")
  const [searchQuery, setSearchQuery] = useState("")
  const [hoveredId, setHoveredId] = useState<number | null>(null)
  const searchInputRef = useRef<HTMLInputElement>(null)

  // Ctrl+F 快捷键聚焦搜索框
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === "f") {
        e.preventDefault()
        searchInputRef.current?.focus()
      }
    }
    window.addEventListener("keydown", handleKeyDown)
    return () => window.removeEventListener("keydown", handleKeyDown)
  }, [])

  return (
    <div className="min-h-screen bg-background flex items-center justify-center p-4">
      <div className="w-full max-w-lg bg-card border border-border rounded-xl shadow-xl overflow-hidden">
        {/* 顶部胶囊拖拽区 */}
        <div className="flex items-center justify-center py-1.5 bg-muted/30 cursor-grab active:cursor-grabbing">
          <div className="w-10 h-1 rounded-full bg-muted-foreground/30" />
        </div>

        {/* 标签栏 */}
        <div className="flex items-center justify-between px-3 py-1.5 border-b border-border">
          <div className="flex items-center gap-1">
            {tabs.map((tab) => (
              <button
                key={tab}
                onClick={() => setActiveTab(tab)}
                className={cn(
                  "px-2.5 py-1 text-xs font-medium rounded-md transition-all",
                  activeTab === tab
                    ? "bg-primary text-primary-foreground"
                    : "text-muted-foreground hover:text-foreground hover:bg-muted"
                )}
              >
                {tab}
              </button>
            ))}
          </div>
          <div className="flex items-center gap-1">
            <button className="p-1.5 rounded-md hover:bg-muted text-muted-foreground hover:text-foreground transition-colors">
              <Bell className="w-3.5 h-3.5" />
            </button>
            <button className="p-1.5 rounded-md hover:bg-muted text-muted-foreground hover:text-foreground transition-colors">
              <Sparkles className="w-3.5 h-3.5" />
            </button>
          </div>
        </div>

        {/* 内容列表 */}
        <div className="max-h-80 overflow-y-auto">
          {mockData.map((item, index) => (
            <div
              key={item.id}
              className="group relative px-3 py-2 border-b border-border/50 hover:bg-muted transition-colors cursor-pointer"
              onMouseEnter={() => setHoveredId(item.id)}
              onMouseLeave={() => setHoveredId(null)}
            >
              <div className="flex items-start gap-2">
                {/* 类型标签 */}
                <span
                  className={cn(
                    "shrink-0 px-1.5 py-0.5 text-[10px] font-medium rounded",
                    item.type === "HTML"
                      ? "bg-blue-500/10 text-blue-600"
                      : item.type === "图片"
                      ? "bg-green-500/10 text-green-600"
                      : "bg-orange-500/10 text-orange-600"
                  )}
                >
                  {item.type}
                </span>

                {/* 内容区域 */}
                <div className="flex-1 min-w-0">
                  <p className="text-sm text-foreground truncate leading-tight">
                    {item.content}
                  </p>
                  <div className="flex items-center gap-2 mt-0.5">
                    <span className="text-[10px] text-muted-foreground">
                      {item.charCount}字符
                    </span>
                    <span className="text-[10px] text-muted-foreground">·</span>
                    <span className="text-[10px] text-muted-foreground">
                      {item.time}
                    </span>
                    {item.starred && (
                      <Star className="w-3 h-3 text-yellow-500 fill-yellow-500" />
                    )}
                  </div>
                </div>

                {/* 操作按钮 */}
                <div
                  className={cn(
                    "flex items-center gap-0.5 transition-opacity",
                    hoveredId === item.id ? "opacity-100" : "opacity-0"
                  )}
                >
                  <button className="p-1 rounded hover:bg-muted text-muted-foreground hover:text-foreground">
                    <Copy className="w-3.5 h-3.5" />
                  </button>
                  <button className="p-1 rounded hover:bg-muted text-muted-foreground hover:text-yellow-500">
                    <Star className="w-3.5 h-3.5" />
                  </button>
                  <button className="p-1 rounded hover:bg-muted text-muted-foreground hover:text-red-500">
                    <Trash2 className="w-3.5 h-3.5" />
                  </button>
                </div>

                {/* 索引编号 */}
                <span className="shrink-0 w-5 text-right text-[10px] text-muted-foreground/60 font-mono tabular-nums">
                  {index + 1}
                </span>
              </div>
            </div>
          ))}
        </div>

        {/* 底部搜索栏 */}
        <div className="px-3 py-2 border-t border-border bg-muted/20">
          <div className="relative">
            <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
            <input
              ref={searchInputRef}
              type="text"
              placeholder="搜索剪贴板..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full h-8 pl-8 pr-16 text-sm bg-muted/50 border-0 rounded-lg focus:outline-none focus:ring-1 focus:ring-primary/50 placeholder:text-muted-foreground"
            />
            <kbd className="absolute right-2.5 top-1/2 -translate-y-1/2 px-1.5 py-0.5 text-[10px] text-muted-foreground bg-muted rounded border border-border font-mono">
              Ctrl+F
            </kbd>
          </div>
        </div>
      </div>
    </div>
  )
}
