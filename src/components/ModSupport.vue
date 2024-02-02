<template>
  <div
      id="modSupport"
      class="modal mod-support"
  >
    <div class="modal-title">
      <i>favorite</i>
      <h5>Support this project</h5>
    </div>

    <div class="mod-support__content">
      <div class="about">
        <p>
          <b>"ufc-ripper"</b> is a free, open-source tool that allows users with a paid subscription to access UFC Fight
          Pass content offline, in the format of their choice. Ideally, such services should offer these features
          without the need for third-party tools. However, since the service provider does not have these features built
          into their platform, the tool was developed to fulfill this need in a completely legal way.
        </p>
        <p>
          It's worth mentioning that it was no easy task creating this tool. It required figuring out how UFC Fight Pass
          API backends work, countless hours of testing, and continuous maintenance for several months to reach this
          point. As the developer of this project, I do not receive any gain from it other than solely being satisfied
          by seeing people use this tool and appreciate it. Therefore, a donation would be greatly appreciated to
          support me in this endeavor.
        </p>
      </div>

      <div class="donation-sources">
        <div class="donation-sources__title">
          <b>You can send a donation my way using these methods:</b>
        </div>

        <div class="donation-sources__links center-content">
          <a
              href="https://ko-fi.variatix.net/"
              target="_blank"
              rel="noopener noreferrer"
          >
            <img
                alt="Ko-fi support button"
                title="Support me on Ko-fi"
                src="@/assets/images/ko-fi.png"
            >
          </a>
          <a
              href="https://paypal.variatix.net/"
              target="_blank"
              rel="noopener noreferrer"
          >
            <img
                alt="Paypal support button"
                title="Support me on Paypal"
                src="@/assets/images/paypal.png"
            >
          </a>
        </div>
      </div>
    </div>

    <nav class="right-align">
      <button data-ui="#modSupport">
        <i>close</i>
        <span>Close</span>
      </button>
    </nav>
  </div>
</template>

<style lang="scss">
.mod-support {
  width: 100%;
  max-width: 500px;
  max-height: 80%;
  display: grid;
  grid-template-rows: max-content minmax(0px, 1fr) max-content;
  overflow: hidden;

  &__content {
    overflow-y: auto;

    .about {
      & > p {
        text-indent: 50px;
        margin-bottom: 20px;
      }
    }

    .donation-sources {
      &__title {
        margin-bottom: 20px;
      }

      &__links {
        margin-bottom: 20px;
        gap: 20px;

        & > a > img {
          border-radius: 0;
        }
      }
    }
  }
}
</style>
