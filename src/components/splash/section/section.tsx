import "./section.css";

export default function SplashSection({
  title,
  children,
}: Readonly<{
  title: string;
  children?: React.ReactNode;
}>) {
  if (!children) {
    return (
      <div className="splash-section">
        <div className="splash-section__title">{title}</div>
      </div>
    );
  }
  return (
    <div className="splash-section">
      <div className="splash-section__title">{title}</div>
      <div className="splash-section__content">
        <div className="splash-section__content__text">
          {children}
        </div>
      </div>
    </div>
  );
}
