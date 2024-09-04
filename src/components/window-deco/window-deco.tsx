"use client";
import "./window-deco.css";
import {
  AdjustmentsHorizontalIcon,
  WindowIcon,
  Square2StackIcon,
  XMarkIcon,
  EllipsisHorizontalIcon,
} from "@heroicons/react/24/solid";
import { invoke } from "@tauri-apps/api/tauri";
import { WebviewWindow } from "@tauri-apps/api/window";
import React, { use, useEffect, useState } from "react";

export default function WindowDeco({
  children,
}: Readonly<{
  children?: React.ReactNode;
}>) {
  const [appWindow, setAppWindow] = useState<WebviewWindow>();

  useEffect(() => {
    (async () => {
      if (window != undefined) {
        const appWindow = (await import("@tauri-apps/api/window")).appWindow;
        setAppWindow(appWindow);
      }
    })();
  });

  return (
    <div className="window-deco">
      <div className="window-deco__title-bar">
        <div className="window-deco__drag-overlay" data-tauri-drag-region />
        <div className="window-deco__title-box">Boxed</div>
      </div>

      <div className="window-deco__title-buttons">
        <button
          className="window-deco__title-button"
          onClick={() => invoke("open_settings")}
        >
          <AdjustmentsHorizontalIcon className="window-deco__title-svg" />
        </button>
        <button
          className="window-deco__title-button"
          onClick={() => {
            appWindow?.minimize();
          }}
        >
          <svg viewBox="-12 -12 24 24" className="window-deco__title-svg">
            <rect
              x="-8"
              y="0"
              width="16"
              height="2"
              stroke="white"
              strokeWidth="0"
              
            />
          </svg>
        </button>
        <button
          className="window-deco__title-button"
          onClick={() => {
            appWindow?.toggleMaximize();
          }}
        >
          <Square2StackIcon className="window-deco__title-svg" />
        </button>
        <button
          className="window-deco__title-button window-deco__title-button-exit"
          onClick={() => {
            appWindow?.close();
          }}
        >
          <XMarkIcon className="window-deco__title-svg" />
        </button>
      </div>
    </div>
  );
}
