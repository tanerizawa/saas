import React from 'react';
import type { Meta, StoryObj } from '@storybook/react';
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  CardFooter
} from '../components/ui/card';
import { Button } from '../components/ui/button';

interface CardDemoProps {
  title?: string;
  description?: string;
  content?: string;
  hasFooter?: boolean;
  footerAlign?: 'left' | 'center' | 'right' | 'space-between';
  variant?: 'default' | 'outlined' | 'elevated';
}

// Wrapper component for Card demo
const CardDemo = ({
  title = 'Card Title',
  description = 'Card description goes here',
  content = 'This is the main content of the card.',
  hasFooter = true,
  footerAlign = 'right',
  variant = 'default',
}: CardDemoProps) => {
  const footerAlignMap = {
    'left': 'justify-start',
    'center': 'justify-center',
    'right': 'justify-end',
    'space-between': 'justify-between',
  };

  const variantStyles = {
    'default': '',
    'outlined': 'shadow-none',
    'elevated': 'shadow-lg',
  };

  return (
    <Card className={`w-[350px] ${variantStyles[variant]}`}>
      <CardHeader>
        {title && <CardTitle>{title}</CardTitle>}
        {description && <CardDescription>{description}</CardDescription>}
      </CardHeader>
      <CardContent>
        <p>{content}</p>
      </CardContent>
      {hasFooter && (
        <CardFooter className={`${footerAlignMap[footerAlign]}`}>
          {footerAlign === 'space-between' ? (
            <>
              <Button variant="outline">Cancel</Button>
              <Button>Submit</Button>
            </>
          ) : (
            <Button>Action</Button>
          )}
        </CardFooter>
      )}
    </Card>
  );
};

const meta: Meta<typeof CardDemo> = {
  title: 'Components/UI/Card',
  component: CardDemo,
  tags: ['autodocs'],
  parameters: {
    layout: 'centered',
  },
  argTypes: {
    title: { control: 'text' },
    description: { control: 'text' },
    content: { control: 'text' },
    hasFooter: { control: 'boolean' },
    footerAlign: {
      control: { type: 'select' },
      options: ['left', 'center', 'right', 'space-between'],
    },
    variant: {
      control: { type: 'select' },
      options: ['default', 'outlined', 'elevated'],
    },
  },
};

export default meta;
type Story = StoryObj<typeof CardDemo>;

export const Default: Story = {
  args: {
    title: 'Default Card',
    description: 'This is a default card with a right-aligned footer',
    content: 'Cards are flexible containers that group related content and actions. They can contain various UI elements like text, buttons, or media.',
  },
};

export const NoFooter: Story = {
  args: {
    title: 'Information Card',
    description: 'This card has no footer',
    content: 'This card is used to display information without any actions.',
    hasFooter: false,
  },
};

export const SpaceBetween: Story = {
  args: {
    title: 'Form Card',
    description: 'This card has a space-between footer for form actions',
    content: 'Form content would typically go here. This example shows how to arrange action buttons with space between them.',
    footerAlign: 'space-between',
  },
};

export const Elevated: Story = {
  args: {
    title: 'Elevated Card',
    description: 'This card has additional elevation',
    content: 'The elevated style adds more shadow to make the card stand out from the background.',
    variant: 'elevated',
  },
};

export const Outlined: Story = {
  args: {
    title: 'Outlined Card',
    description: 'This card has a border but no shadow',
    content: 'The outlined style is more subtle and flat, useful for secondary content.',
    variant: 'outlined',
  },
};
