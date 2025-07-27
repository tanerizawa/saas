import React from 'react';
import type { Meta, StoryObj } from '@storybook/react';
import { 
  Toast, 
  ToastAction, 
  ToastClose,
  ToastDescription,
  ToastProvider,
  ToastTitle,
  ToastViewport
} from '../components/ui/toast';
import { Button } from '../components/ui/button';

interface ToastDemoProps {
  variant?: "default" | "destructive";
  title?: string;
  description?: string;
  action?: string;
  hasCloseButton?: boolean;
  duration?: number;
  position?: "top-right" | "top-left" | "bottom-right" | "bottom-left";
}

// Wrapper component for Toast demo
const ToastDemo = ({
  variant = "default",
  title = "Toast Title",
  description = "Toast description goes here",
  action,
  hasCloseButton = true,
  position = "bottom-right",
  duration = 5000,
}: ToastDemoProps) => {
  const [open, setOpen] = React.useState(false);

  const positionStyles: Record<string, string> = {
    "top-right": "top-0 right-0",
    "top-left": "top-0 left-0",
    "bottom-right": "bottom-0 right-0",
    "bottom-left": "bottom-0 left-0",
  };

  return (
    <div className="flex flex-col items-center gap-2">
      <Button onClick={() => setOpen(true)}>Show Toast</Button>

      <ToastProvider duration={duration}>
        <Toast 
          variant={variant} 
          open={open} 
          onOpenChange={setOpen}
        >
          <div className="grid gap-1">
            {title && <ToastTitle>{title}</ToastTitle>}
            {description && <ToastDescription>{description}</ToastDescription>}
          </div>
          {action && (
            <ToastAction altText={action}>
              {action}
            </ToastAction>
          )}
          {hasCloseButton && <ToastClose />}
        </Toast>
        <ToastViewport className={positionStyles[position]} />
      </ToastProvider>
    </div>
  );
};

const meta: Meta<typeof ToastDemo> = {
  title: 'Components/UI/Toast',
  component: ToastDemo,
  tags: ['autodocs'],
  parameters: {
    layout: 'centered',
  },
  argTypes: {
    variant: {
      control: { type: 'select' },
      options: ['default', 'destructive'],
    },
    title: { control: 'text' },
    description: { control: 'text' },
    action: { control: 'text' },
    hasCloseButton: { control: 'boolean' },
    duration: { control: 'number' },
    position: {
      control: { type: 'select' },
      options: ['top-right', 'top-left', 'bottom-right', 'bottom-left'],
    },
  },
};

export default meta;
type Story = StoryObj<typeof ToastDemo>;

export const Default: Story = {
  args: {
    title: 'Notification',
    description: 'Your message has been sent successfully.',
  },
};

export const WithAction: Story = {
  args: {
    title: 'Undo Changes',
    description: 'Your changes have been saved.',
    action: 'Undo',
  },
};

export const Destructive: Story = {
  args: {
    variant: 'destructive',
    title: 'Error',
    description: 'Something went wrong. Please try again.',
    action: 'Try again',
  },
};

export const TopRight: Story = {
  args: {
    title: 'Top Right Toast',
    description: 'This toast appears in the top right corner.',
    position: 'top-right',
  },
};

export const LongDuration: Story = {
  args: {
    title: 'Long Duration',
    description: 'This toast will stay visible for 10 seconds.',
    duration: 10000,
  },
};
