export default function AboutMe() {
  return (
    <div className="pt-10 px-10">
      <div className="grid grid-cols-[25%_70%] gap-x-10">
        <img
          // format from here: https://stackoverflow.com/questions/10311092/displaying-files-e-g-images-stored-in-google-drive-on-a-website
          src="https://lh3.googleusercontent.com/d/1pO_ty9Cs7ZkRTOOrmRTa_lzkxgPWrVfU"
          // this is needed otherwise we get a 429 status code: https://stackoverflow.com/questions/79052869/google-drive-returns-429-when-using-saved-photo-as-src-for-img-tag
          referrerPolicy="no-referrer"
          alt="display picture"
          width="200"
          height="200"
          className="inline"
        />
        <span className="flex justify-center items-center">
          <div className="h-auto w-auto">
            <Header1 content="About Me" />
            <p className="mt-5">
              Hi, I'm Sunny. I graduated from Software Engineering at UNSW in
              December of 2024. I'm currently a course developer and tutor for
              the Advanced C++ Programming Course. This website showcases some
              of my coding projects and blogs. Enjoy!
            </p>
          </div>
        </span>
      </div>
      <div className="mt-7.5">
        <Header1 content="What I've Worked On" />
        <p>
          I've done fullstack development for various early-stage startups with
          B2B SAS products in areas ranging from accounting to project
          management. This involved working closely with founders to create
          features from inception to implementation - i.e. from mockup to a
          staging environment release.
        </p>
      </div>
      <div className="mt-5">
        <Header1 content="Technical Expertise" />
        <p>
          I love learning new languages and technologies. Here is a
          comprehensive list of the ones I've learnt over my programming
          journey.
        </p>
        <ol>
          <ListItemWithBoldPrefix
            bold="General-Purpose Languages"
            normal="C, Java, C++, Python, Rust"
          />
          <ListItemWithBoldPrefix
            bold="Scripting"
            normal="Bash, Powershell, Perl"
          />
          <ListItemWithBoldPrefix
            bold="Front-End Development"
            normal="HTML, CSS, Vanilla JS, Typescript, React JS, Tailwind CSS"
          />
        </ol>
      </div>
      <div className="mt-5">
        <Header1 content="Links" />
        <p>
          Contact:&#32;
          <a href="mailto:sunny.chen1@unswalumni.com">
            sunny.chen1@unswalumni.com
          </a>
        </p>
      </div>
    </div>
  );
}

function Header1({ content }) {
  return <h1 className="text-[1.5em]">{content}</h1>;
}

function ListItemWithBoldPrefix({ bold, normal }) {
  return (
    <li>
      <b>{bold}</b>: {normal}
    </li>
  );
}
