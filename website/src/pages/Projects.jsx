import Header1 from "../components/Header1";

import ProjectPreviewCard from "../cards/ProjectPreviewCard";

export default function Projects() {
  return (
    <div>
      <div className="flex justify-center items-center h-[calc(1.5em_+20px)] pt-[20px]">
        <Header1 content="My Projects" />
      </div>
      <div className="mt-10 w-full flex flex-col items-center">
        <FocusTrackerPreview />
      </div>
    </div>
  );
}

function FocusTrackerPreview() {
  return (
    <ProjectPreviewCard
      title="Focus Tracker"
      dates="2023"
      description="An educational tool that executes a Python program and traces significant points in execution. Fith-Year Software Engineering thesis project."
      technologies="Backend written in Python with unit and integration testing done in Pytest. Frontend written in React JS and vanilla CSS."
    />
  );
}
