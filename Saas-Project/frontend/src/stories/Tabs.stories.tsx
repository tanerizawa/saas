import React from 'react';
import type { Meta, StoryObj } from '@storybook/react';
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger
} from '../components/ui/tabs';

interface TabsExampleProps {
  defaultValue?: string;
  orientation?: 'horizontal' | 'vertical';
  tabs?: Array<{ value: string, label: string, content: string }>;
  className?: string;
}

// Wrapper component for Tabs
const TabsExample = ({
  defaultValue = 'tab1',
  orientation = 'horizontal',
  tabs = [
    { value: 'tab1', label: 'Tab 1', content: 'Tab 1 Content' },
    { value: 'tab2', label: 'Tab 2', content: 'Tab 2 Content' },
    { value: 'tab3', label: 'Tab 3', content: 'Tab 3 Content' },
  ],
  className,
}: TabsExampleProps) => {
  return (
    <Tabs defaultValue={defaultValue} className={className}>
      <TabsList className={orientation === 'vertical' ? 'flex-col h-auto' : ''}>
        {tabs.map((tab) => (
          <TabsTrigger key={tab.value} value={tab.value}>
            {tab.label}
          </TabsTrigger>
        ))}
      </TabsList>
      {tabs.map((tab) => (
        <TabsContent key={tab.value} value={tab.value}>
          <div className="p-4 border rounded-md mt-2">{tab.content}</div>
        </TabsContent>
      ))}
    </Tabs>
  );
};

const meta: Meta<typeof TabsExample> = {
  title: 'Components/UI/Tabs',
  component: TabsExample,
  tags: ['autodocs'],
  parameters: {
    layout: 'centered',
  },
  argTypes: {
    defaultValue: { control: 'text' },
    orientation: {
      control: { type: 'radio' },
      options: ['horizontal', 'vertical'],
    },
    tabs: { control: 'object' },
  },
};

export default meta;
type Story = StoryObj<typeof TabsExample>;

export const Default: Story = {
  args: {
    tabs: [
      { value: 'account', label: 'Account', content: 'Make changes to your account.' },
      { value: 'password', label: 'Password', content: 'Change your password here.' },
      { value: 'settings', label: 'Settings', content: 'Manage your account settings.' },
    ],
    defaultValue: 'account',
  },
};

export const Vertical: Story = {
  args: {
    tabs: [
      { value: 'account', label: 'Account', content: 'Make changes to your account.' },
      { value: 'password', label: 'Password', content: 'Change your password here.' },
      { value: 'settings', label: 'Settings', content: 'Manage your account settings.' },
    ],
    defaultValue: 'account',
    orientation: 'vertical',
    className: 'flex flex-row gap-4',
  },
};

export const WithIcons: Story = {
  args: {
    tabs: [
      { 
        value: 'overview', 
        label: '📊 Overview', 
        content: 'This tab shows an overview of your analytics data.' 
      },
      { 
        value: 'analytics', 
        label: '📈 Analytics', 
        content: 'Detailed analytics about your account activity and performance metrics.' 
      },
      { 
        value: 'reports', 
        label: '📑 Reports', 
        content: 'Download and manage your account reports.' 
      },
    ],
    defaultValue: 'overview',
  },
};
