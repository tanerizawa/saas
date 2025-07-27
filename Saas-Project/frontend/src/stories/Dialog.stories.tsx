import React from 'react';
import type { Meta, StoryObj } from '@storybook/react';
import { Button } from '../components/ui/button';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '../components/ui/dialog';

// Compound component wrapper for Dialog
const DialogExample = ({
  title = 'Dialog Title',
  description = 'This is a dialog description that explains the dialog content.',
  triggerText = 'Open Dialog',
  hasFooter = true,
  footerContent = 'Default Footer Content',
}: {
  title?: string;
  description?: string;
  triggerText?: string;
  hasFooter?: boolean;
  footerContent?: string | React.ReactNode;
}) => {
  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="outline">{triggerText}</Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
          <DialogDescription>{description}</DialogDescription>
        </DialogHeader>
        <div className="py-4">
          This is the main content of the dialog. You can put any components or text here.
        </div>
        {hasFooter && (
          <DialogFooter>
            {typeof footerContent === 'string' ? (
              <Button type="submit">Save changes</Button>
            ) : (
              footerContent
            )}
          </DialogFooter>
        )}
      </DialogContent>
    </Dialog>
  );
};

const meta: Meta<typeof DialogExample> = {
  title: 'Components/UI/Dialog',
  component: DialogExample,
  tags: ['autodocs'],
  parameters: {
    layout: 'centered',
  },
  argTypes: {
    title: { control: 'text' },
    description: { control: 'text' },
    triggerText: { control: 'text' },
    hasFooter: { control: 'boolean' },
    footerContent: { control: 'text' },
  },
};

export default meta;
type Story = StoryObj<typeof DialogExample>;

export const Default: Story = {
  args: {
    title: 'Edit Profile',
    description: 'Make changes to your profile here. Click save when you\'re done.',
    triggerText: 'Edit Profile',
  },
};

export const WithoutFooter: Story = {
  args: {
    title: 'Information',
    description: 'This is an informational dialog without action buttons.',
    triggerText: 'Show Info',
    hasFooter: false,
  },
};

export const DeleteConfirmation: Story = {
  args: {
    title: 'Are you sure?',
    description: 'This action cannot be undone. This will permanently delete your account and remove your data from our servers.',
    triggerText: 'Delete Account',
    footerContent: (
      <>
        <Button variant="outline">Cancel</Button>
        <Button variant="destructive">Delete</Button>
      </>
    ),
  },
  parameters: {
    docs: {
      description: {
        story: 'A confirmation dialog with a destructive action button.',
      },
    },
  },
};
