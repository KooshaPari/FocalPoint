export interface HeroSection {
  tagline: string;
  headline: string;
  subheading: string;
  cta_primary: string;
  cta_secondary: string;
  cta_primary_url: string;
  cta_secondary_url: string;
}

export interface FeatureItem {
  title: string;
  description: string;
}

export interface Connector {
  name: string;
  icon: string;
}

export interface FeaturesSection {
  title: string;
  connectors: Connector[];
  grid: FeatureItem[];
}

export interface HowItWorksStep {
  number: string;
  title: string;
  description: string;
}

export interface HowItWorksSection {
  title: string;
  subtitle: string;
  steps: HowItWorksStep[];
}

export interface PricingTier {
  name: string;
  price: string;
  billing: string;
  description: string;
  features: string[];
  cta: string;
  highlighted: boolean;
}

export interface PricingSection {
  title: string;
  subtitle: string;
  tiers: PricingTier[];
}

export interface TestimonialsSection {
  title: string;
  subtitle: string;
  note: string;
}

export interface LegalLink {
  name: string;
  url: string;
}

export interface CommunityLink {
  name: string;
  url: string;
}

export interface FooterSection {
  tagline: string;
  legal: LegalLink[];
  community: CommunityLink[];
  copyright: string;
}
