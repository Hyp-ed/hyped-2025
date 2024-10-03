# End to End Testing Briefing

Full list: [Issue #102](https://github.com/Hyp-ed/hyped-2024/issues/102)

<ol>
<li><span class="totest">All inputs on the sidebar, including setting levitation height, are visible with correct labels and the correct MQTT state transition message is emitted (for the GO button, make sure that the enable toggle works)
</span>
<details>
  <summary>Issues</summary>
  <ul>
    <li>Setting Levitation Height</li>
    <li>Correct Labels Visible</li>
    <li>MQTT State Transition Message Emitted (<em>Enable Toggle should work for 'GO' button</em>)</li>
  </ul>
</details>

</li>
<hr>

<li><span class="totest">Switching between pods changes the selected pod for certain elements</span>
<details>
  <summary>Issues</summary>
  <ul>
    <li>Setting Levitation Height</li>
    <li>Correct Labels Visible</li>
    <li>MQTT State Transition Message Emitted (<em>Enable Toggle should work for 'GO' button</em>)</li>
  </ul>
</details>
</li>
<hr>

<li><span class="totest">All view options are listed, visible and render the correct component</span>
<details>
  <summary>Issues</summary>
  <ul>
    <li>Setting Levitation Height</li>
    <li>Correct Labels Visible</li>
    <li>MQTT State Transition Message Emitted (<em>Enable Toggle should work for 'GO' button</em>)</li>
  </ul>
</details>
</li>
<hr>

<li><span class="totest">State machine diagram renders with the expected nodes visible</span>
<details>
  <summary>Issues</summary>
  <ul>
  </ul>
</details>
</li>
<hr>

<li><span class="totest">Open MCT plots measurements on graphs correctly (historical test)</span>
<details>
  <summary>Issues</summary>
  <ul>
  </ul>
</details>
</li>
<hr>

<li><span class="totest">Open MCT plots live measurements from MQTT messages (realtime test)</span>
<details>
  <summary>Issues</summary>
  <ul>
  </ul>
</details>
</li>
<hr>

<li><span class="totest">Open MCT logs a fault when a measurement limit is exceeded (check warning and critical limits)</span>
<details>
  <summary>Issues</summary>
  <ul>
  </ul>
</details>
</li>
<hr>
</ol>

<span>More to come...</span>

</ol>

<style>
  .totest {
    color: #FF7F7F;
  }
  .doing {
    color: #00BFFF;
    font-weight: bold;
  }
  .done {
    color: lime;
    text-decoration: underline;
  }
  summary {
    font-size: 14px;
    font-weight: bold;
  }
</style>
