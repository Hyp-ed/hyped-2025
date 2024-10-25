import { Linkedin, Facebook, Instagram, Github, Twitter } from 'lucide-react';
import Link from 'next/link';

/**
 * Defines the social media icons and their links.
 */
const SOCIAL_ICONS = {
  facebook: {
    link: 'https://www.facebook.com/hypedinburgh/',
    component: (
      <Facebook className="hover:scale-110 transition text-hyped-red" />
    ),
  },
  github: {
    link: 'https://github.com/hyp-ed/hyped-2024',
    component: <Github className="hover:scale-110 transition text-hyped-red" />,
  },
  instagram: {
    link: 'https://www.instagram.com/hypedinburgh/',
    component: (
      <Instagram className="hover:scale-110 transition text-hyped-red" />
    ),
  },
  linkedIn: {
    link: 'https://www.linkedin.com/company/hyp-ed/',
    component: (
      <Linkedin className="hover:scale-110 transition text-hyped-red" />
    ),
  },
  twitter: {
    link: 'https://twitter.com/hyped_hyperloop',
    component: (
      <Twitter className="hover:scale-110 transition text-hyped-red" />
    ),
  },
};

/**
 * Displays a row of social media icons that link to the respective social media pages.
 * @returns The social media icons.
 */
export const SocialIcons = () => {
  const socialIcons = Object.values(SOCIAL_ICONS);

  return (
    <div className="flex flex-row justify-between w-1/2 sm:max-w-sm">
      {socialIcons.map(({ link, component }, index) => (
        <Link href={link} key={index}>
          {component}
        </Link>
      ))}
    </div>
  );
};
