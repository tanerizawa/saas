import React from 'react';
import type { Meta, StoryObj } from '@storybook/react';
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
  SelectSeparator
} from '../components/ui/select';

type Option = { value: string; label: string };
type GroupOption = { label: string; items: Option[] };

// This wrapper is needed because Select is a compound component
const SelectComponent = ({
  placeholder = 'Select an option',
  disabled = false,
  options = [
    { value: 'apple', label: 'Apple' },
    { value: 'banana', label: 'Banana' },
    { value: 'orange', label: 'Orange' },
  ],
  groups = [] as GroupOption[],
  showSeparator = false,
}) => {
  return (
    <Select disabled={disabled}>
      <SelectTrigger className="w-[180px]">
        <SelectValue placeholder={placeholder} />
      </SelectTrigger>
      <SelectContent>
        {groups.length > 0 ? (
          groups.map((group, i) => (
            <React.Fragment key={group.label}>
              <SelectGroup>
                <SelectLabel>{group.label}</SelectLabel>
                {group.items.map((item) => (
                  <SelectItem key={item.value} value={item.value}>
                    {item.label}
                  </SelectItem>
                ))}
              </SelectGroup>
              {i < groups.length - 1 && showSeparator && <SelectSeparator />}
            </React.Fragment>
          ))
        ) : (
          options.map((option) => (
            <SelectItem key={option.value} value={option.value}>
              {option.label}
            </SelectItem>
          ))
        )}
      </SelectContent>
    </Select>
  );
};

const meta: Meta<typeof SelectComponent> = {
  title: 'Components/UI/Select',
  component: SelectComponent,
  tags: ['autodocs'],
  parameters: {
    layout: 'centered',
  },
  argTypes: {
    disabled: { control: 'boolean' },
    placeholder: { control: 'text' },
    showSeparator: { control: 'boolean' },
    options: { control: 'object' },
    groups: { control: 'object' }
  },
};

export default meta;
type Story = StoryObj<typeof SelectComponent>;

export const Default: Story = {
  args: {
    placeholder: 'Select a fruit',
    options: [
      { value: 'apple', label: 'Apple' },
      { value: 'banana', label: 'Banana' },
      { value: 'orange', label: 'Orange' },
    ],
  },
};

export const Disabled: Story = {
  args: {
    placeholder: 'Select a fruit',
    disabled: true,
    options: [
      { value: 'apple', label: 'Apple' },
      { value: 'banana', label: 'Banana' },
      { value: 'orange', label: 'Orange' },
    ],
  },
};

export const WithGroups: Story = {
  args: {
    placeholder: 'Select a food',
    showSeparator: true,
    groups: [
      {
        label: 'Fruits',
        items: [
          { value: 'apple', label: 'Apple' },
          { value: 'banana', label: 'Banana' },
          { value: 'orange', label: 'Orange' },
        ],
      },
      {
        label: 'Vegetables',
        items: [
          { value: 'carrot', label: 'Carrot' },
          { value: 'broccoli', label: 'Broccoli' },
          { value: 'spinach', label: 'Spinach' },
        ],
      },
    ],
  },
};
