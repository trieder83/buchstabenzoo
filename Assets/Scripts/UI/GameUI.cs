using UnityEngine;

public class GameUI : MonoBehaviour
{
    private GUIStyle labelStyle;
    private GUIStyle buttonStyle;

    void Start()
    {
        labelStyle = new GUIStyle();
        labelStyle.fontSize = 24;
        labelStyle.fontStyle = FontStyle.Bold;
        labelStyle.normal.textColor = Color.white;

        buttonStyle = new GUIStyle("button");
        buttonStyle.fontSize = 18;
    }

    void OnGUI()
    {
        // Title
        GUI.Label(new Rect(20, 10, 400, 40), "Buchstaben Zoo", labelStyle);

        // Difficulty selector
        var gm = GameManager.Instance;
        if (gm == null) return;

        GUI.Label(new Rect(20, 50, 200, 30), $"Stufe: {gm.GetDifficulty()}", labelStyle);

        float btnY = 90;
        if (GUI.Button(new Rect(20, btnY, 160, 35), "Kindergarten", buttonStyle))
            gm.currentDifficulty = GameManager.Difficulty.Kindergarten;
        if (GUI.Button(new Rect(190, btnY, 100, 35), "Klasse 1", buttonStyle))
            gm.currentDifficulty = GameManager.Difficulty.Grade1;
        if (GUI.Button(new Rect(300, btnY, 100, 35), "Klasse 2", buttonStyle))
            gm.currentDifficulty = GameManager.Difficulty.Grade2;
        if (GUI.Button(new Rect(410, btnY, 100, 35), "Klasse 3", buttonStyle))
            gm.currentDifficulty = GameManager.Difficulty.Grade3;

        // Instructions
        labelStyle.fontSize = 16;
        GUI.Label(new Rect(20, Screen.height - 40, 600, 30),
            "Tippe ein Tier an und ziehe es in das richtige Gehege!", labelStyle);
        labelStyle.fontSize = 24;
    }
}
