export type SavedResultForm = {
  status: "editing" | "solved";
};

export type SavedResultEntry<TForm extends SavedResultForm> = {
  id: string;
  form: TForm;
  createdAt: number;
};
