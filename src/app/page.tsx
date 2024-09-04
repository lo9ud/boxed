import SplashSection from "@/components/splash/section/section";
import SplashNewSelector from "@/components/splash/splash-new-selector/splash-new-selector";
import SplashOpenSelector from "@/components/splash/splash-open-selector/splash-open-selector";

export default function Home() {
  return (
    <main>
      <SplashSection title="New">
        <SplashNewSelector/>
      </SplashSection>
      <SplashSection title="Open">
        <SplashOpenSelector/>
      </SplashSection>
    </main>
  );
}
