import type { Metadata } from "next";
import "./globals.css";
import "../components/window-deco/window-deco";
import WindowDeco from "@/components/window-deco/window-deco";

export const metadata: Metadata = {
  title: "Boxed",
  description: "A tabulation software package",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>
        <WindowDeco />
        {children}
      </body>
    </html>
  );
}
